use std::sync::Arc;

use futures_util::lock::Mutex;
use serde::{Serialize, Serializer, ser::SerializeMap};

// Timestamp abstraction to support both chrono and jiff
#[cfg(all(feature = "jiff-0_2", not(feature = "chrono")))]
use jiff::Timestamp;
#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};

use crate::{
    Response, ServerResult, Value,
    extensions::{
        Extension, ExtensionContext, ExtensionFactory, NextExecute, NextResolve, ResolveInfo,
    },
    value,
};

// Type alias for timestamp type based on enabled features
#[cfg(all(feature = "jiff-0_2", not(feature = "chrono")))]
type TimestampType = Timestamp;
#[cfg(feature = "chrono")]
type TimestampType = DateTime<Utc>;

// Helper functions to abstract timestamp operations
#[cfg(all(feature = "jiff-0_2", not(feature = "chrono")))]
mod timestamp_ops {
    use jiff::Timestamp;

    pub fn now() -> Timestamp {
        Timestamp::now()
    }

    pub fn to_rfc3339(ts: &Timestamp) -> String {
        ts.to_string()
    }

    pub fn duration_nanos(start: &Timestamp, end: &Timestamp) -> Option<i64> {
        let duration = end.duration_since(*start);
        // SignedDuration::as_nanos() returns i128, convert to i64
        duration.as_nanos().try_into().ok()
    }
}

#[cfg(feature = "chrono")]
mod timestamp_ops {
    use chrono::{DateTime, Utc};

    pub fn now() -> DateTime<Utc> {
        Utc::now()
    }

    pub fn to_rfc3339(ts: &DateTime<Utc>) -> String {
        ts.to_rfc3339()
    }

    pub fn duration_nanos(start: &DateTime<Utc>, end: &DateTime<Utc>) -> Option<i64> {
        (*end - *start).num_nanoseconds()
    }
}

struct ResolveState {
    path: Vec<String>,
    field_name: String,
    parent_type: String,
    return_type: String,
    start_time: TimestampType,
    end_time: TimestampType,
    start_offset: i64,
}

impl Serialize for ResolveState {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("path", &self.path)?;
        map.serialize_entry("fieldName", &self.field_name)?;
        map.serialize_entry("parentType", &self.parent_type)?;
        map.serialize_entry("returnType", &self.return_type)?;
        map.serialize_entry("startOffset", &self.start_offset)?;
        map.serialize_entry(
            "duration",
            &timestamp_ops::duration_nanos(&self.start_time, &self.end_time),
        )?;
        map.end()
    }
}

/// Apollo tracing extension for performance tracing
///
/// Apollo Tracing works by including data in the extensions field of the
/// GraphQL response, which is reserved by the GraphQL spec for extra
/// information that a server wants to return. That way, you have access to
/// performance traces alongside the data returned by your query. It's already
/// supported by `Apollo Engine`, and we're excited to see what other kinds of
/// integrations people can build on top of this format.
#[cfg_attr(docsrs, doc(cfg(feature = "apollo_tracing")))]
pub struct ApolloTracing;

impl ExtensionFactory for ApolloTracing {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(ApolloTracingExtension {
            inner: Mutex::new(Inner {
                start_time: timestamp_ops::now(),
                end_time: timestamp_ops::now(),
                resolves: Default::default(),
            }),
        })
    }
}

struct Inner {
    start_time: TimestampType,
    end_time: TimestampType,
    resolves: Vec<ResolveState>,
}

struct ApolloTracingExtension {
    inner: Mutex<Inner>,
}

#[async_trait::async_trait]
impl Extension for ApolloTracingExtension {
    async fn execute(
        &self,
        ctx: &ExtensionContext<'_>,
        operation_name: Option<&str>,
        next: NextExecute<'_>,
    ) -> Response {
        self.inner.lock().await.start_time = timestamp_ops::now();
        let resp = next.run(ctx, operation_name).await;

        let mut inner = self.inner.lock().await;
        inner.end_time = timestamp_ops::now();
        inner
            .resolves
            .sort_by(|a, b| a.start_offset.cmp(&b.start_offset));
        resp.extension(
            "tracing",
            value!({
                "version": 1,
                "startTime": timestamp_ops::to_rfc3339(&inner.start_time),
                "endTime": timestamp_ops::to_rfc3339(&inner.end_time),
                "duration": timestamp_ops::duration_nanos(&inner.start_time, &inner.end_time),
                "execution": {
                    "resolvers": inner.resolves
                }
            }),
        )
    }

    async fn resolve(
        &self,
        ctx: &ExtensionContext<'_>,
        info: ResolveInfo<'_>,
        next: NextResolve<'_>,
    ) -> ServerResult<Option<Value>> {
        let path = info.path_node.to_string_vec();
        let field_name = info.path_node.field_name().to_string();
        let parent_type = info.parent_type.to_string();
        let return_type = info.return_type.to_string();
        let start_time = timestamp_ops::now();
        let query_start_time = self.inner.lock().await.start_time;
        let start_offset = timestamp_ops::duration_nanos(&query_start_time, &start_time)
            .unwrap_or(0);

        let res = next.run(ctx, info).await;
        let end_time = timestamp_ops::now();

        self.inner.lock().await.resolves.push(ResolveState {
            path,
            field_name,
            parent_type,
            return_type,
            start_time,
            end_time,
            start_offset,
        });
        res
    }
}

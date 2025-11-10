#![cfg(feature = "jiff-0_2")]

use async_graphql::*;

#[tokio::test]
async fn test_jiff_datetime_scalar() {
    struct Query;

    #[Object]
    impl Query {
        async fn get_datetime(&self) -> jiff::Zoned {
            "2024-01-15T10:30:00Z".parse().unwrap()
        }
    }

    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    let query = r#"{ __type(name: "JiffDateTime") { name } }"#;
    let res = schema.execute(query).await.into_result().unwrap().data;

    assert_eq!(
        res,
        value!({
            "__type": {
                "name": "JiffDateTime"
            }
        })
    );
}

#[tokio::test]
async fn test_jiff_civil_date_scalar() {
    struct Query;

    #[Object]
    impl Query {
        async fn get_date(&self) -> jiff::civil::Date {
            "2024-01-15".parse().unwrap()
        }
    }

    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    let query = r#"{ __type(name: "JiffDate") { name } }"#;
    let res = schema.execute(query).await.into_result().unwrap().data;

    assert_eq!(
        res,
        value!({
            "__type": {
                "name": "JiffDate"
            }
        })
    );
}

#[tokio::test]
async fn test_jiff_civil_time_scalar() {
    struct Query;

    #[Object]
    impl Query {
        async fn get_time(&self) -> jiff::civil::Time {
            "10:30:00".parse().unwrap()
        }
    }

    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    let query = r#"{ __type(name: "JiffTime") { name } }"#;
    let res = schema.execute(query).await.into_result().unwrap().data;

    assert_eq!(
        res,
        value!({
            "__type": {
                "name": "JiffTime"
            }
        })
    );
}

#[tokio::test]
async fn test_jiff_civil_datetime_scalar() {
    struct Query;

    #[Object]
    impl Query {
        async fn get_datetime(&self) -> jiff::civil::DateTime {
            "2024-01-15T10:30:00".parse().unwrap()
        }
    }

    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    let query = r#"{ __type(name: "JiffCivilDateTime") { name } }"#;
    let res = schema.execute(query).await.into_result().unwrap().data;

    assert_eq!(
        res,
        value!({
            "__type": {
                "name": "JiffCivilDateTime"
            }
        })
    );
}

#[tokio::test]
async fn test_jiff_span_scalar() {
    struct Query;

    #[Object]
    impl Query {
        async fn get_duration(&self) -> jiff::Span {
            "P1DT2H30M".parse().unwrap()
        }
    }

    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    let query = r#"{ __type(name: "JiffSpan") { name } }"#;
    let res = schema.execute(query).await.into_result().unwrap().data;

    assert_eq!(
        res,
        value!({
            "__type": {
                "name": "JiffSpan"
            }
        })
    );
}

#[tokio::test]
#[cfg(feature = "jiff-tz")]
async fn test_jiff_timezone_scalar() {
    struct Query;

    #[Object]
    impl Query {
        async fn get_timezone(&self) -> jiff::tz::TimeZone {
            jiff::tz::TimeZone::get("America/New_York").unwrap()
        }
    }

    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    let query = r#"{ __type(name: "JiffTimeZone") { name } }"#;
    let res = schema.execute(query).await.into_result().unwrap().data;

    assert_eq!(
        res,
        value!({
            "__type": {
                "name": "JiffTimeZone"
            }
        })
    );
}

#[tokio::test]
async fn test_jiff_datetime_parse_and_serialize() {
    struct Query;

    #[Object]
    impl Query {
        async fn echo_datetime(&self, dt: jiff::Zoned) -> jiff::Zoned {
            dt
        }
    }

    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    let query = r#"{ echoDatetime(dt: "2024-01-15T10:30:00Z") }"#;
    let res = schema.execute(query).await;

    if !res.is_ok() {
        eprintln!("Error: {:?}", res.errors);
    }
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_jiff_date_parse_and_serialize() {
    struct Query;

    #[Object]
    impl Query {
        async fn echo_date(&self, d: jiff::civil::Date) -> jiff::civil::Date {
            d
        }
    }

    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    let query = r#"{ echoDate(d: "2024-01-15") }"#;
    let res = schema.execute(query).await;

    assert!(res.is_ok());
}

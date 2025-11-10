use jiff::{Timestamp, Zoned};

use crate::{InputValueError, InputValueResult, Scalar, ScalarType, Value};

/// Implement the Zoned scalar
///
/// The input/output is a string in RFC3339 format.
/// Jiff's Zoned type can represent timestamps with any timezone (UTC, fixed offset, or IANA timezone).
/// For parsing RFC3339 strings without explicit timezone info, defaults to UTC.
#[Scalar(
    internal,
    name = "JiffDateTime",
    specified_by_url = "https://datatracker.ietf.org/doc/html/rfc3339"
)]
impl ScalarType for Zoned {
    fn parse(value: Value) -> InputValueResult<Self> {
        match &value {
            Value::String(s) => {
                // First try parsing as Zoned (handles formats like "2024-01-15T10:30:00[UTC]")
                if let Ok(zoned) = s.parse::<Zoned>() {
                    return Ok(zoned);
                }

                // Fall back to parsing as Timestamp (handles RFC3339 like "2024-01-15T10:30:00Z")
                // and convert to Zoned in UTC
                s.parse::<Timestamp>()
                    .map(|ts| ts.to_zoned(jiff::tz::TimeZone::UTC))
                    .map_err(|e| InputValueError::custom(e.to_string()))
            }
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> Value {
        // Use intz() to get RFC3339 format with timezone offset (e.g., "2024-01-15T10:30:00Z")
        Value::String(self.timestamp().to_string())
    }
}

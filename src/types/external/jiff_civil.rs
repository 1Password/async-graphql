use jiff::civil::{Date, DateTime, Time};

use crate::{InputValueError, InputValueResult, Scalar, ScalarType, Value};

#[Scalar(internal, name = "JiffDate")]
/// ISO 8601 calendar date without timezone.
/// Format: YYYY-MM-DD
///
/// # Examples
///
/// * `1994-11-13`
/// * `2000-02-24`
impl ScalarType for Date {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(s) => s
                .parse::<Date>()
                .map_err(|e| InputValueError::custom(e.to_string())),
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.to_string())
    }
}

#[Scalar(internal, name = "JiffTime")]
/// ISO 8601 time without timezone.
/// Allows for nanosecond precision.
/// Format: HH:MM:SS[.fraction]
///
/// # Examples
///
/// * `08:59:60.123`
/// * `12:30:00`
impl ScalarType for Time {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(s) => s
                .parse::<Time>()
                .map_err(|e| InputValueError::custom(e.to_string())),
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.to_string())
    }
}

#[Scalar(internal, name = "JiffCivilDateTime")]
/// ISO 8601 combined date and time without timezone.
///
/// # Examples
///
/// * `2015-07-01T08:59:60.123`
/// * `2020-01-01T00:00:00`
impl ScalarType for DateTime {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(s) => s
                .parse::<DateTime>()
                .map_err(|e| InputValueError::custom(e.to_string())),
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.to_string())
    }
}

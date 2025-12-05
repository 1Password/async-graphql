use jiff::tz::TimeZone;

use crate::{InputValueError, InputValueResult, Scalar, ScalarType, Value};

#[Scalar(
    internal,
    name = "JiffTimeZone",
    specified_by_url = "http://www.iana.org/time-zones"
)]
impl ScalarType for TimeZone {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(s) => TimeZone::get(&s)
                .map_err(|e| InputValueError::custom(e.to_string())),
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.iana_name().unwrap_or("UTC").to_owned())
    }
}

use jiff::Span;

use crate::{InputValueError, InputValueResult, Scalar, ScalarType, Value};

/// Implement the Span scalar
///
/// The input/output is a string in ISO8601 format.
/// Jiff's Span type represents a duration with calendar-aware units (years, months, days, hours, etc.)
#[Scalar(
    internal,
    name = "JiffSpan",
    specified_by_url = "https://en.wikipedia.org/wiki/ISO_8601#Durations"
)]
impl ScalarType for Span {
    fn parse(value: Value) -> InputValueResult<Self> {
        match &value {
            Value::String(s) => s
                .parse::<Span>()
                .map_err(|e| InputValueError::custom(e.to_string())),
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.to_string())
    }
}

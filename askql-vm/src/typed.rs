use crate::r#type::{ScalarType, TypedValue};
use askql_parser::{AskCodeOrValue, Value};
use serde::Serialize;

pub fn typed(value: AskCodeOrValue, r#type: Option<ScalarType>) -> TypedValue {
    todo!();
}
pub fn untyped(value: AskCodeOrValue) -> Value {
    match value {
        AskCodeOrValue::Value(value) => value,
        AskCodeOrValue::AskCode(code) => {
            todo!();
        }
    }
}
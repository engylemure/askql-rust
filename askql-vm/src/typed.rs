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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_untyped() {
        let code_or_value = AskCodeOrValue::new_value(Value::Null);
        dbg!(untyped(code_or_value));
    }
    #[test]
    fn test_untyped2() {}
}

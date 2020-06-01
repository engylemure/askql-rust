use askql_parser::{AskCode, AskCodeOrValue, Number, Value};
use serde::{Deserialize, Serialize};

pub trait Type: Serialize + std::fmt::Debug {
    fn name() -> String;
    fn validate<T: Type>(value: T) -> bool {
        true
    }
}

impl<T: Type> Type for Option<T> {
    fn name() -> String {
        "empty".to_string()
    }

    fn validate<U: Type>(_value: U) -> bool {
        Self::name() == U::name()
    }
}

impl<T: Type> Type for &T {
    fn name() -> String {
        T::name()
    }

    fn validate<U: Type>(value: U) -> bool {
        T::validate(value)
    }
}

impl Type for bool {
    fn name() -> String {
        "boolean".to_string()
    }

    fn validate<U: Type>(_value: U) -> bool {
        Self::name() == U::name()
    }
}

impl Type for String {
    fn name() -> String {
        "string".to_string()
    }

    fn validate<U: Type>(_value: U) -> bool {
        String::name() == U::name()
    }
}

impl Type for &str {
    fn name() -> String {
        "string".to_string()
    }

    fn validate<U: Type>(_value: U) -> bool {
        String::name() == U::name()
    }
}

impl Type for i32 {
    fn name() -> String {
        "int".to_string()
    }

    fn validate<U: Type>(_value: U) -> bool {
        i32::name() == U::name()
    }
}

impl Type for f32 {
    fn name() -> String {
        "float".to_string()
    }

    fn validate<U: Type>(_value: U) -> bool {
        f32::name() == U::name()
    }
}

impl Type for Number {
    fn name() -> String {
        "number".to_string()
    }

    fn validate<U: Type>(_value: U) -> bool {
        Number::name() == U::name()
    }
}

impl Type for AskCode {
    fn name() -> String {
        "code".to_string()
    }

    fn validate<U: Type>(_value: U) -> bool {
        AskCode::name() == U::name()
    }
}

pub struct TypedValue {
    pub r#type: ScalarType,
    pub value: Value,
}

impl TypedValue {
    pub fn new(r#type: ScalarType, value: Value) -> Self {
        Self { r#type, value }
    }
}

#[derive(Debug, Serialize)]
pub enum ScalarType {
    Null,
    Boolean,
    Int,
    Float,
    List,
    Any,
    Code,
    Other(String),
}

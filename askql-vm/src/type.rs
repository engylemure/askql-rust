use askql_parser::AskCode;
use serde::{Serialize, Deserialize};

pub trait Type {
    fn name() -> String;
    fn validate<T: Type>(value: T) -> bool {
        true
    }
}

impl<T> Type for Option<T> {
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
        "code".to_strign()
    }

    fn validate<U: Type>(_value: U) -> bool {
        AskCode::name() == U::name()
    }
}


#[derive(Debug)]
pub enum Types {
    Empty,
    Boolean(bool),
    Int(i32),
    Float(f32),
    Number(Number),
    String(String),
    Any(Box<dyn Type + Serialize + Deserialize + Debug>)
}

pub struct Number(String);
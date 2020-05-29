use crate::reduce::Reducer;
use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AskCodeOrValue {
    Value(Value),
    AskCode(AskCode)
}

impl AskCodeOrValue {
    pub fn new_value(value: Value) -> AskCodeOrValue {
        AskCodeOrValue::Value(value)
    }

    pub fn new_ask_code(askcode: AskCode) -> AskCodeOrValue {
        AskCodeOrValue::AskCode(askcode)
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Value {
    Null,
    Boolean(bool),
    Number(String),
    String(String),
    Object(BTreeMap<String, Value>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AskCode {
    pub name: String,
    pub params: Option<Vec<AskCodeOrValue>>
}

impl AskCode {
    pub fn new(name: String, params: Option<Vec<AskCodeOrValue>>) -> Self {
        Self {
            name,
            params
        }
    }
}
pub trait AskCodeTrait: std::fmt::Debug + Clone + Sized {
    fn name(&self) -> String;
    fn params(&self) -> Option<Vec<AskCodeOrValue>>;
}

impl AskCodeTrait for AskCode {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn params(&self) -> Option<Vec<AskCodeOrValue>> {
        self.params.clone()
    }
}

pub struct AskCodeReducer {}

impl Reducer<AskCodeOrValue> for AskCodeReducer {
    fn node(&self, name: String, children: Option<Vec<AskCodeOrValue>>) -> AskCodeOrValue {
        AskCodeOrValue::new_ask_code(AskCode { name, params: children })
    }
    fn id(&self, name: String) -> AskCodeOrValue {
        AskCodeOrValue::new_ask_code(AskCode { name, params: None })
    }
    fn value(&self, value: crate::reduce::Value) -> AskCodeOrValue {
        match value {
            crate::reduce::Value::Null => AskCodeOrValue::new_value(Value::Null),
            crate::reduce::Value::Boolean(bool) => AskCodeOrValue::new_value(Value::Boolean(bool)),
            crate::reduce::Value::Number(num) => AskCodeOrValue::new_value(Value::Number(num.0)),
            crate::reduce::Value::String(string) => AskCodeOrValue::new_value(Value::String(string)),
        }
    }
}

pub fn is_ask_code<T: AskCodeTrait>(value: &AskCodeOrValue) -> bool {
    use AskCodeOrValue::*;
    match value {
        AskCode(_) => true,
        _ => false,
    }
}

pub fn is_value<T: AskCodeTrait>(value: &AskCodeOrValue) -> bool {
    use AskCodeOrValue::*;
    match value {
        AskCode(_) => false,
        _ => true,
    }
}

pub fn ask_code_to_source(value: &AskCodeOrValue) -> String {
    match value {
        AskCodeOrValue::Value(value) => match value {
            Value::String(string) => string.clone(),
            val => format!("{}", val),
        },
        AskCodeOrValue::AskCode(askcode) => {
            let params = match askcode.params() {
                Some(params) => params
                    .iter()
                    .map(ask_code_to_source)
                    .collect::<Vec<String>>()
                    .join(","),
                None => "".to_string(),
            };
            format!(
                "{} {{
                {}
            }}",
                askcode.name(),
                params
            )
        }
    }
}

#[derive(Debug)]
pub enum AskCordOrValueError {
    ExpectingValue,
    ExpectingAskCode,
}

pub fn ask_code(value: &AskCodeOrValue) -> Result<AskCode, AskCordOrValueError> {
    match value {
        AskCodeOrValue::AskCode(code) => Ok(code.clone()),
        _ => Err(AskCordOrValueError::ExpectingAskCode),
    }
}

pub fn value(value: &AskCodeOrValue) -> Result<Value, AskCordOrValueError> {
    match value {
        AskCodeOrValue::Value(value) => Ok(value.clone()),
        AskCodeOrValue::AskCode(_) => Err(AskCordOrValueError::ExpectingValue),
    }
}

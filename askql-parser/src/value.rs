use regex::Regex;
use serde::{
    ser::{SerializeMap, SerializeSeq, SerializeStruct},
    Serialize, Serializer,
};
use std::collections::BTreeMap;
use std::fmt;

#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub enum Value {
    Null,
    Boolean(bool),
    Int(i32),
    Float(f32),
    Number(Number),
    String(String),
    Object(BTreeMap<String, Value>),
    List(Vec<Value>),
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::Null => serializer.serialize_none(),
            Value::String(s) => serializer.serialize_str(s),
            Value::Number(n) => {
                if n.is_float() {
                    if let Ok(float) = n.to_float() {
                        return serializer.serialize_f32(float);
                    }
                } else {
                    if let Ok(int) = n.to_int() {
                        return serializer.serialize_i32(int);
                    }
                }
                serializer.serialize_i32(0)
            }
            Value::Int(i) => serializer.serialize_i32(*i),
            Value::Float(float) => serializer.serialize_f32(*float),
            Value::Boolean(b) => serializer.serialize_bool(*b),
            Value::List(list) => {
                let mut seq = serializer.serialize_seq(Some(list.len()))?;
                for e in list {
                    seq.serialize_element(e)?;
                }
                seq.end()
            }
            Value::Object(obj) => {
                let mut map = serializer.serialize_map(Some(obj.len()))?;
                for (k, v) in obj {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Null
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        use Value::*;
        let tuple = (self, other);
        match tuple {
            (Null, Null) => true,
            (Boolean(a), Boolean(b)) => a == b,
            (Int(a), Int(b)) => a == b,
            (Float(a), Float(b)) => a == b,
            (Number(a), Number(b)) => a == b,
            (String(a), String(b)) => a == b,
            (Object(a), Object(b)) => a == b,
            (List(a), List(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for Value {}

impl Value {
    pub fn null() -> Self {
        Value::Null
    }
    pub fn boolean(val: bool) -> Self {
        Value::Boolean(val)
    }
    pub fn int(val: i32) -> Self {
        Value::Int(val)
    }
    pub fn float(val: f32) -> Self {
        Value::Float(val)
    }
    pub fn string(val: String) -> Self {
        Value::String(val)
    }
    pub fn number(val: String) -> Self {
        Value::Number(Number(val))
    }
    pub fn is_object(&self) -> bool {
        if let Value::Object(_) = self {
            true
        } else {
            false
        }
    }
    pub fn is_list(&self) -> bool {
        if let Value::List(_) = self {
            true
        } else {
            false
        }
    }
    pub fn is_null(&self) -> bool {
        match self {
            Value::Null => true,
            _ => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Number(pub String);

pub enum NumberConvertError {
    NotANumber,
}

impl Number {
    pub fn is_int(&self) -> bool {
        !self.is_float()
    }
    pub fn is_float(&self) -> bool {
        lazy_static! {
            static ref IS_FLOAT_REGEX: Regex = Regex::new(r#"^[-+]?\d*\.\d*$"#).unwrap();
        }
        IS_FLOAT_REGEX.is_match(&self.0)
    }
    pub fn to_int(&self) -> Result<i32, NumberConvertError> {
        use NumberConvertError::*;
        self.0.parse().map_err(|_| NotANumber)
    }
    pub fn to_float(&self) -> Result<f32, NumberConvertError> {
        use NumberConvertError::*;
        self.0.parse().map_err(|_| NotANumber)
    }
}

impl From<Number> for i32 {
    fn from(value: Number) -> i32 {
        todo!()
    }
}

impl From<Number> for f32 {
    fn from(value: Number) -> f32 {
        todo!()
    }
}

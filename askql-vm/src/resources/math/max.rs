use crate::resource::Resource;
use crate::run::AskVm;
use askql_parser::{AskCode, AskCodeOrValue, Value};
use async_trait::async_trait;

pub struct MaxResource;

pub fn flatten(args: Vec<Value>) -> Vec<Value> {
    args.into_iter().fold(Vec::new(), |mut acc, val| {
        match val {
            Value::List(list) => acc.extend(flatten(list)),
            Value::Object(obj) => {
                let mut flattened = Vec::new();
                for (k, v) in obj.into_iter() {
                    flattened.push(v);
                }
                acc.extend(flattened)
            }
            val => acc.push(val),
        };
        acc
    })
}

#[async_trait]
impl Resource for MaxResource {
    fn name(&self) -> String {
        "max".to_string()
    }
    async fn resolver(&self, args: Vec<Value>) -> Value {
        if args.len() > 0 {
            let flattened = flatten(args);
            flattened.into_iter().fold(Value::Null, |acc, val| {
                if acc.is_null() {
                    match val {
                        Value::Int(int) => Value::Int(int),
                        Value::Float(float) => Value::Float(float),
                        _ => acc,
                    }
                } else {
                    match (acc, val) {
                        (Value::Int(a), Value::Int(b)) => Value::Int(if b > a { b } else { a }),
                        (Value::Int(a), Value::Float(b)) => {
                            if b > a as f32 {
                                Value::Float(b)
                            } else {
                                Value::Int(a)
                            }
                        }
                        (Value::Float(a), Value::Int(b)) => {
                            if b as f32 > a {
                                Value::Int(b)
                            } else {
                                Value::Float(a)
                            }
                        }
                        (Value::Float(a), Value::Float(b)) => {
                            Value::Float(if b > a { b } else { a })
                        }
                        (acc, _) => acc,
                    }
                }
            })
        } else {
            Value::Null
        }
    }
}

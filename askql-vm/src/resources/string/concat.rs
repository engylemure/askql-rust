use crate::resource::Resource;
use crate::run::AskVm;
use askql_parser::{AskCode, AskCodeOrValue, Value};
use async_trait::async_trait;

pub struct ConcatResource;

#[async_trait]
impl Resource for ConcatResource {
    fn name(&self) -> String {
        "concat".to_string()
    }
    async fn resolver(&self, args: Vec<Value>) -> Value {
        let string = args
            .into_iter()
            .fold(String::new(), |mut acc, val| match val {
                Value::Int(integer) => format!("{}{}", acc, integer),
                Value::Float(float) => format!("{}{}", acc, float),
                Value::String(string) => {
                    acc.push_str(&string);
                    acc
                }
                _ => acc,
            });
        Value::String(string)
    }
}

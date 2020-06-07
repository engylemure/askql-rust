use crate::resource::Resource;
use crate::run::AskVm;
use askql_parser::{AskCode, AskCodeOrValue, Value};
use async_trait::async_trait;

pub struct ToUpperCaseResource;

#[async_trait]
impl Resource for ToUpperCaseResource {
    fn name(&self) -> String {
        "toUpperCase".to_string()
    }
    async fn resolver(&self, mut args: Vec<Value>) -> Value {
        if args.len() > 0 {
            match args.remove(0) {
                Value::String(string) => Value::String(string.to_uppercase()),
                value => value,
            }
        } else {
            Value::Null
        }
    }
}

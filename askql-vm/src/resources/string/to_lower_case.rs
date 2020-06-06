use crate::resource::Resource;
use crate::run::AskVm;
use askql_parser::{AskCode, AskCodeOrValue, Value};
use async_trait::async_trait;

pub struct ToLowerCaseResource;

#[async_trait]
impl Resource for ToLowerCaseResource {
    fn name(&self) -> String {
        "toLowerCase".to_string()
    }
    async fn resolver(&self, mut args: Vec<Value>) -> Value {
        if args.len() > 0 {
            match args.remove(0) {
                Value::String(string) => Value::String(string.to_lowercase()),
                value => value
            }
        } else {
            Value::Null
        }
    }
}

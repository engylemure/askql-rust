use crate::resource::Resource;
use crate::run::AskVm;
use askql_parser::{AskCode, AskCodeOrValue, Value};
use async_trait::async_trait;

pub struct ListResource;

#[async_trait]
impl Resource for ListResource {
    fn name(&self) -> String {
        "list".to_string()
    }
    async fn resolver(&self, args: Vec<Value>) -> Value {
        Value::List(args)
    }
}

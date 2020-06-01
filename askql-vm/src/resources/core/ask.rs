use super::fun::FunResource;
use crate::resource::Resource;
use crate::run::AskVm;
use askql_parser::{AskCode, Value};
use async_trait::async_trait;

pub struct AskResource;

#[async_trait]
impl Resource for AskResource {
    fn name(&self) -> String {
        "ask".to_string()
    }
    async fn compute(&self, vm: &AskVm, code: AskCode, args: Option<Vec<Value>>) -> Value {
        FunResource::compute(self, vm, code, args).await
    }
}

#[async_trait]
impl FunResource for AskResource {}

use crate::resource::Resource;
use crate::run::AskVm;
use askql_parser::{AskCode, AskCodeOrValue, Value};
use async_trait::async_trait;
use super::fun::FunResource;

pub struct FragmentResource;

#[async_trait]
impl Resource for FragmentResource {
    fn name(&self) -> String {
        "f".to_string()
    }
    async fn compute(&self, vm: &AskVm, code: AskCode, args: Option<Vec<Value>>) -> Value {
        FunResource::compute(self, vm, code, args).await
    }
}

#[async_trait]
impl FunResource for FragmentResource {}
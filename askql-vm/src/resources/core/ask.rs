use super::fun::FunResource;
use crate::resource::Resource;
use crate::run::AskVm;
use askql_parser::*;
use async_trait::async_trait;
use std::collections::HashMap;

pub struct AskResource;

#[async_trait]
impl Resource for AskResource {
    fn name(&self) -> String {
        "ask".to_string()
    }
    async fn compute(&self, vm: &AskVm, code: AskCode, args: Option<Vec<Value>>, extended_options: Option<HashMap<String, AskCodeOrValue>>) -> Value {
        FunResource::compute(self, vm, code, args, extended_options).await
    }
}

#[async_trait]
impl FunResource for AskResource {}

use crate::run::{AskVm, RunOptions};
use askql_parser::*;
use async_trait::async_trait;
use std::collections::HashMap;
use std::marker::{Send, Sync};

#[async_trait]
pub trait Resource: Sync + Send {
    fn name(&self) -> String;
    async fn resolver(&self, args: Vec<Value>) -> Value {
        Value::Null
    }
    async fn compute(
        &self,
        vm: &AskVm,
        code: AskCode,
        args: Option<Vec<Value>>,
        extended_options: Option<HashMap<String, AskCodeOrValue>>,
    ) -> Value {
        let args = match args {
            Some(args) => args,
            None => {
                let mut args = Vec::new();
                for param in code.params.unwrap_or(vec![]) {
                    if let Ok(value) = vm.run(param, None, None).await {
                        args.push(value)
                    }
                }
                args
            }
        };
        self.resolver(args).await
    }
}

use crate::run::{AskVm, RunOptions};
use askql_parser::{AskCode, Value};
use async_trait::async_trait;
use std::marker::{Send, Sync};

#[async_trait]
pub trait Resource: Sync + Send {
    fn name(&self) -> String;
    async fn resolver(&self, args: Vec<Value>) -> Value;
    async fn compute(&self, vm: &AskVm, code: AskCode, args: Option<Vec<Value>>) -> Value {
        let args = match args {
            Some(args) => args,
            None => {
                let mut args = Vec::new();
                for param in code.params.unwrap_or(vec![]) {
                    if let Ok(value) = vm.run(param, None).await {
                        args.push(value)
                    }
                }
                args
            }
        };
        self.resolver(args).await
    }
}

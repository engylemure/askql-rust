use crate::resource::Resource;
use crate::run::AskVm;
use askql_parser::{AskCode, Value};
use async_trait::async_trait;

#[async_trait]
pub trait FunResource: Resource {
    async fn compute(&self, vm: &AskVm, code: AskCode, args: Option<Vec<Value>>) -> Value {
        match args {
            Some(args) => {
                let AskCode { params, .. } = code;
                let mut last_result = Value::Null;
                if let Some(statements) = params {
                    for statement in statements {
                        if let Ok(value) = vm.run(statement, None).await {
                            last_result = value;
                        }
                    }
                }
                last_result
            }
            None => {
                let AskCode { params, .. } = code;
                let mut last_result = Value::Null;
                if let Some(statements) = params {
                    for statement in statements {
                        if let Ok(value) = vm.run(statement, None).await {
                            last_result = value;
                        }
                    }
                }
                last_result
            },
        }
    }
}

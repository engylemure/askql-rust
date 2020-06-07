use crate::resource::Resource;
use crate::run::AskVm;
use askql_parser::*;
use async_trait::async_trait;
use futures::future::join_all;
use std::collections::HashMap;

#[async_trait]
pub trait FunResource: Resource {
    async fn compute(
        &self,
        vm: &AskVm,
        code: AskCode,
        args: Option<Vec<Value>>,
        extended_options: Option<HashMap<String, AskCodeOrValue>>,
    ) -> Value {
        match args {
            Some(args) => {
                let AskCode { params, .. } = code;
                let mut last_result = Value::Null;
                let cloned_opts = extended_options.clone();
                if let Some(statements) = params {
                    for statement in statements {
                        if let Ok(value) = vm.run(statement, None, cloned_opts.clone()).await {
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
                    let cloned_opts = extended_options.clone();
                    let statements = join_all(
                        statements
                            .into_iter()
                            .map(move |statement| vm.run(statement, None, cloned_opts.clone())),
                    )
                    .await;
                    for statement in statements {
                        if let Ok(value) = statement {
                            last_result = value;
                        }
                    }
                }
                last_result
            }
        }
    }
}

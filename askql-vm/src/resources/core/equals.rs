use super::fun::FunResource;
use crate::resource::Resource;
use crate::run::AskVm;
use askql_parser::*;
use async_trait::async_trait;
use std::collections::HashMap;

pub struct EqualsResource;

#[async_trait]
impl Resource for EqualsResource {
    fn name(&self) -> String {
        "equals".to_string()
    }
    async fn resolver(&self, args: Vec<Value>) -> Value {
        let (is_equal, _) =
            args.into_iter()
                .enumerate()
                .fold((true, Value::Null), |mut acc, (idx, val)| {
                    if idx == 0 {
                        acc.1 = val;
                    } else {
                        acc.0 = acc.1 == val;
                        acc.1 = val;
                    }
                    acc
                });
        Value::Boolean(is_equal)
    }
}

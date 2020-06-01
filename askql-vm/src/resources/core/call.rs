use crate::resource::Resource;
use crate::run::AskVm;
use askql_parser::{AskCode, AskCodeOrValue, Value};
use async_trait::async_trait;

pub struct CallResource;

#[async_trait]
impl Resource for CallResource {
    fn name(&self) -> String {
        "call".to_string()
    }
    async fn compute(&self, vm: &AskVm, code: AskCode, args: Option<Vec<Value>>) -> Value {
        let AskCode { name, params } = code;
        let mut statements = params
            .or(args.map(|args| {
                args.into_iter()
                    .map(|arg| AskCodeOrValue::Value(arg)).collect()
            }))
            .unwrap_or(vec![]);
        if statements.len() > 0 {
            let arg_children: Vec<AskCodeOrValue> = statements.drain(1..).collect();
            let fun_child = statements.remove(0);
            let mut args = Vec::new();
            for arg in arg_children {
                if let Ok(arg) = vm.run(arg, None).await {
                    args.push(arg);
                }
            }
            if let Ok(result) = vm.run(fun_child, Some(args)).await {
                return result;
            }
        }
        Value::Null
    }
}

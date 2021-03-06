use crate::resource::Resource;
use crate::run::AskVm;
use askql_parser::{AskCode, AskCodeOrValue, Value};
use async_trait::async_trait;
use std::collections::HashMap;
pub struct GetResource;

#[async_trait]
impl Resource for GetResource {
    fn name(&self) -> String {
        "get".to_string()
    }
    async fn compute(
        &self,
        vm: &AskVm,
        code: AskCode,
        args: Option<Vec<Value>>,
        extended_options: Option<HashMap<String, AskCodeOrValue>>,
    ) -> Value {
        let AskCode { name, params } = code;
        if let Some(mut params) = params {
            if let AskCodeOrValue::Value(Value::String(name)) = params.remove(0) {
                return vm
                    .run(
                        AskCodeOrValue::AskCode(AskCode::new(name, None)),
                        args,
                        extended_options,
                    )
                    .await
                    .unwrap_or(Value::Null);
            }
        }
        Value::Null
    }
}

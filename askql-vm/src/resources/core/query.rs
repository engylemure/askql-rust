use crate::resource::Resource;
use crate::run::AskVm;
use askql_parser::{AskCode, AskCodeOrValue, Value};
use async_trait::async_trait;
use futures::future::join_all;
use std::collections::HashMap;

pub struct NodeResource;

impl NodeResource {
    async fn process(
        &self,
        vm: &AskVm,
        value: AskCodeOrValue,
        children: Vec<AskCodeOrValue>,
    ) -> Value {
        match value {
            AskCodeOrValue::Value(value) => {
                let cloned_val = value.clone();
                match value {
                    Value::Object(obj) => {
                        let futures = children.into_iter().map(|child| match child {
                            AskCodeOrValue::AskCode(code) => {
                                async {
                                    let AskCode {
                                        name: code_name,
                                        params,
                                    } = code;
                                    match params {
                                        Some(mut params) => {
                                            let name_getter = params[0].clone();
                                            let name = vm
                                                .run(
                                                    name_getter,
                                                    Some(vec![cloned_val.clone()]),
                                                    None,
                                                )
                                                .await
                                                .unwrap_or_default();
                                            let code = AskCodeOrValue::AskCode(AskCode::new(
                                                code_name,
                                                Some(params),
                                            ));
                                            // dbg!(&cloned_val);
                                            let value = vm
                                                .run(code, Some(vec![cloned_val.clone()]), None)
                                                .await
                                                .unwrap_or_default();
                                            return Value::List(vec![name, value]);
                                        }
                                        None => Value::Null,
                                    }
                                }
                            }
                            _ => panic!(),
                        });
                        Value::Object(
                            join_all(futures)
                                .await
                                .into_iter()
                                .map(|val| match val {
                                    Value::List(mut entries) => {
                                        match (entries.remove(0), entries.remove(0)) {
                                            (Value::String(key), val) => Some((key, val)),
                                            _ => None,
                                        }
                                    }
                                    _ => None,
                                })
                                .filter_map(|v| v)
                                .collect(),
                        )
                    }
                    val => val,
                }
            }
            code => {
                dbg!("Holla holla!");
                vm.run(code, None, None).await.unwrap_or_default()
            }
        }
    }

    fn args_to_extended_options(
        &self,
        args: Option<Vec<Value>>,
    ) -> Option<std::collections::HashMap<String, AskCodeOrValue>> {
        match args {
            Some(mut args) => {
                if args.len() > 0 {
                    match args.remove(0) {
                        Value::Object(obj) => Some(
                            obj.into_iter()
                                .map(|(k, v)| (k, AskCodeOrValue::Value(v)))
                                .collect(),
                        ),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

#[async_trait]
impl Resource for NodeResource {
    fn name(&self) -> String {
        "node".to_string()
    }
    async fn compute(
        &self,
        vm: &AskVm,
        code: AskCode,
        args: Option<Vec<Value>>,
        extended_options: Option<HashMap<String, AskCodeOrValue>>,
    ) -> Value {
        let AskCode { name, params } = code;
        match params {
            Some(mut params) if params.len() >= 1 => {
                let children: Vec<AskCodeOrValue> = params.drain(2..).collect();
                let extended_options = self.args_to_extended_options(args);
                // dbg!(&extended_options);
                let value_getter: AskCodeOrValue = params.remove(1);
                // dbg!(&value_getter);
                let value = vm
                    .run(value_getter, None, extended_options)
                    .await
                    .unwrap_or_default();
                // dbg!(&value);
                if let Value::List(list) = value {
                    Value::List(
                        join_all(list.into_iter().map(|v| {
                            self.process(vm, AskCodeOrValue::new_value(v), children.clone())
                        }))
                        .await,
                    )
                } else {
                    self.process(vm, AskCodeOrValue::new_value(value), children)
                        .await
                }
            }
            _ => Value::Null,
        }
    }
}

pub struct QueryResource(NodeResource);

impl QueryResource {
    pub fn new() -> Self {
        Self(NodeResource {})
    }
}

#[async_trait]
impl Resource for QueryResource {
    fn name(&self) -> String {
        "query".to_string()
    }
    async fn compute(
        &self,
        vm: &AskVm,
        code: AskCode,
        args: Option<Vec<Value>>,
        extended_options: Option<HashMap<String, AskCodeOrValue>>,
    ) -> Value {
        let AskCode { name, params } = code;
        return self
            .0
            .compute(
                vm,
                AskCode::new(
                    "node".to_string(),
                    params.map(|children| {
                        let mut params = vec![
                            AskCodeOrValue::Value(Value::String("value".to_string())),
                            AskCodeOrValue::Value(Value::Object(std::collections::BTreeMap::new())),
                        ];
                        params.extend(children);
                        params
                    }),
                ),
                None,
                extended_options,
            )
            .await;
    }
}

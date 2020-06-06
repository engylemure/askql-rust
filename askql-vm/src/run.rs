use crate::r#type::ScalarType;
use crate::resource::Resource;
use crate::typed::{typed, untyped};
use askql_parser::{AskCode, AskCodeOrValue, Value};
use std::boxed::Box;
use std::collections::HashMap;
use std::sync::Arc;
use futures::future::{BoxFuture, FutureExt};

pub struct RunOptions {
    pub resources: HashMap<String, Box<dyn Resource>>,
    pub values: HashMap<String, AskCodeOrValue>,
}

impl RunOptions {
    pub fn new(resources: Vec<Box<dyn Resource>>, values: HashMap<String, AskCodeOrValue>) -> Self {
        Self {
            resources: resources
                .into_iter()
                .map(|resource| (resource.name(), resource))
                .collect(),
            values,
        }
    }
}

pub struct AskVm {
    options: Arc<RunOptions>,
}

impl AskVm {
    pub fn new(options: RunOptions) -> Self {
        Self {
            options: Arc::new(options),
        }
    }

    pub fn run(&self, code: AskCodeOrValue, args: Option<Vec<Value>>, extended_options: Option<HashMap<String, AskCodeOrValue>>) -> BoxFuture<Result<Value, ()>> {
        let options = self.options.clone();
        async move {
            match code {
                AskCodeOrValue::Value(Value::Number(number)) => {
                    if number.is_float() {
                        Ok(Value::Float(number.to_float().unwrap_or(0.0)))
                    } else {
                        Ok(Value::Int(number.to_int().unwrap_or(0)))
                    }
                },
                AskCodeOrValue::Value(value) => Ok(value),
                AskCodeOrValue::AskCode(code) => {
                    // dbg!(&code.name);
                    if let Some(ext_opt) = &extended_options {
                        match ext_opt.get(&code.name) {
                            Some(value) => return self.run(value.clone(), args, None).await,
                            None => {
                                // dbg!("Errr");
                            }
                        };
                    }
                    match options.resources.get(&code.name) {
                        Some(resource) => {
                            Ok(resource.compute(self, code, args, extended_options).await)
                        },
                        None => match options.values.get(&code.name) {
                            Some(value) => self.run(value.clone(), args, None).await,
                            None => {
                                // dbg!("Errr1");
                                return Err(())
                            }
                        },
                    }
                }
            }
        }.boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::*;
    use askql_parser::{AskCodeOrValue, Value};

    #[tokio::test]
    async fn basic_run() {
        let vm = AskVm::new(RunOptions::new(vec![], HashMap::new()));
        let code = AskCodeOrValue::Value(askql_parser::Value::Null);
        assert_eq!(vm.run(code, None, None).await, Ok(Value::Null))
    }
    #[tokio::test]
    async fn sum_operation() {
        let ask_resource = AskResource {};
        let call_resource = CallResource {};
        let get_resource = GetResource {};
        let sum_resource = SumResource {};
        let resources: Vec<Box<dyn crate::resource::Resource>> = vec![
            Box::new(ask_resource),
            Box::new(call_resource),
            Box::new(get_resource),
            Box::new(sum_resource),
        ];
        let vm = AskVm::new(RunOptions::new(resources, HashMap::new()));
        let ask_code = "ask(call(get('+'),2,3,4,5.2))";
        let code = askql_parser::parse(ask_code.to_string(), false).unwrap();
        let result = vm.run(code, None, None).await;
        assert_eq!(Ok(Value::Float(14.2)), result);
    }

    #[tokio::test]
    async fn minus_operation() {
        let ask_resource = AskResource {};
        let call_resource = CallResource {};
        let get_resource = GetResource {};
        let sum_resource = SumResource {};
        let minus_resource = MinusResource {};
        let resources: Vec<Box<dyn crate::resource::Resource>> = vec![
            Box::new(ask_resource),
            Box::new(call_resource),
            Box::new(get_resource),
            Box::new(sum_resource),
            Box::new(minus_resource),
        ];
        let vm = AskVm::new(RunOptions::new(resources, HashMap::new()));
        let ask_code = "ask(call(get('-'),2,3,4,5.2))";
        let code = askql_parser::parse(ask_code.to_string(), false).unwrap();
        let result = vm.run(code, None, None).await;
        assert_eq!(Ok(Value::Float(-14.2)), result);
    }

    #[tokio::test]
    async fn chained_operation() {
        let ask_resource = AskResource {};
        let call_resource = CallResource {};
        let get_resource = GetResource {};
        let sum_resource = SumResource {};
        let minus_resource = MinusResource {};
        let resources: Vec<Box<dyn crate::resource::Resource>> = vec![
            Box::new(ask_resource),
            Box::new(call_resource),
            Box::new(get_resource),
            Box::new(sum_resource),
            Box::new(minus_resource),
        ];
        let vm = AskVm::new(RunOptions::new(resources, HashMap::new()));
        let ask_code = "ask(call(get('-'),call(get('-'),2,3,4,5.2), call(get('+'),2,3,4,5.2)))";
        let code = askql_parser::parse(ask_code.to_string(), false).unwrap();
        let result = vm.run(code, None, None).await;
        assert_eq!(Ok(Value::Int(0)), result);
    }
}

use crate::r#type::ScalarType;
use crate::resource::Resource;
use crate::typed::{typed, untyped};
use askql_parser::{AskCode, AskCodeOrValue, Value};
use std::collections::HashMap;
use std::sync::Arc;

pub struct RunOptions {
    pub resources: HashMap<String, Box<dyn Resource>>,
}

impl RunOptions {
    pub fn new(resources: Vec<Box<dyn Resource>>) -> Self {
        Self {
            resources: resources
                .into_iter()
                .map(|resource| (resource.name(), resource))
                .collect(),
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

    pub async fn run(&self, code: AskCodeOrValue, args: Option<Vec<Value>>) -> Result<Value, ()> {
        match code {
            AskCodeOrValue::Value(value) => Ok(value),
            AskCodeOrValue::AskCode(code) => {
                let options = &self.options.clone();
                match options.resources.get(&code.name) {
                    Some(resource) => Ok(resource.compute(self, code, args).await),
                    None => return Err(()),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use askql_parser::AskCodeOrValue;

    #[tokio::test]
    async fn basic_run() {
        let vm = AskVm::new(RunOptions::new(vec![]));
        let code = AskCodeOrValue::Value(askql_parser::Value::Null);
        dbg!(vm.run(code, None).await);
    }
    #[tokio::test]
    async fn basic_operation() {
        let vm = AskVm::new(RunOptions::new(vec![]));
        let ask_code = "ask(call(get('+'),2,3))";
        let code = askql_parser::parse(ask_code.to_string(), false).unwrap();
        dbg!(vm.run(code, None).await);
    }
}

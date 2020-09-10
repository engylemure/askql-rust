use crate::r#type::ScalarType;
use crate::resource::Resource;
use crate::typed::{typed, untyped};
use askql_parser::{AskCode, AskCodeOrValue, Value};
use futures::future::{BoxFuture, FutureExt};
use std::boxed::Box;
use std::collections::HashMap;
use std::sync::Arc;

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

    pub fn register<T: Resource + 'static>(&mut self, resource: T) -> Option<T> {
        let name = resource.name();
        self.register_with_name(resource, name)
    }

    pub fn register_with_name<T: Resource + 'static>(&mut self, resource: T, name: String) -> Option<T> {
        if self.resources.contains_key(&name) {
            Some(resource)
        } else {
            self.resources.insert(name, Box::new(resource));
            None
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

    pub fn run(
        &self,
        code: AskCodeOrValue,
        args: Option<Vec<Value>>,
        extended_options: Option<HashMap<String, AskCodeOrValue>>,
    ) -> BoxFuture<Result<Value, ()>> {
        let options = self.options.clone();
        async move {
            match code {
                AskCodeOrValue::Value(Value::Number(number)) => {
                    if number.is_float() {
                        Ok(Value::Float(number.to_float().unwrap_or(0.0)))
                    } else {
                        Ok(Value::Int(number.to_int().unwrap_or(0)))
                    }
                }
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
                        }
                        None => match options.values.get(&code.name) {
                            Some(value) => self.run(value.clone(), args, None).await,
                            None => {
                                // dbg!("Errr1");
                                return Err(());
                            }
                        },
                    }
                }
            }
        }
        .boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::*;
    use askql_parser::{AskCodeOrValue, Value};

    fn new_vm(resources: Vec<Box<dyn crate::resource::Resource>>, values: HashMap<String, AskCodeOrValue>) -> AskVm {
        let mut run_options = RunOptions::new(resources, values);
        run_options.register(AskResource);
        run_options.register(CallResource);
        run_options.register(GetResource);
        run_options.register(SumResource);
        run_options.register(MinusResource);
        run_options.register(TimesResource);
        run_options.register(ConcatResource);
        run_options.register(MaxResource);
        run_options.register(ListResource);
        run_options.register(NodeResource);
        run_options.register(QueryResource::new());
        run_options.register(FragmentResource);
        run_options.register(ToLowerCaseResource);
        run_options.register(ToUpperCaseResource);
        AskVm::new(run_options)
    }

    #[tokio::test]
    async fn basic_run() {
        let vm = new_vm(vec![], HashMap::new());
        let code = AskCodeOrValue::Value(askql_parser::Value::Null);
        assert_eq!(vm.run(code, None, None).await, Ok(Value::Null))
    }
    #[tokio::test]
    async fn sum_operation() {
        let vm = new_vm(vec![], HashMap::new());
        let ask_code = "ask(call(get('+'),2,3,4,5.2))";
        let code = askql_parser::parse(ask_code.to_string(), false).unwrap();
        let result = vm.run(code, None, None).await;
        assert_eq!(Ok(Value::Float(14.2)), result);
    }

    #[tokio::test]
    async fn minus_operation() {
        let vm = new_vm(vec![], HashMap::new());
        let ask_code = "ask(call(get('-'),-2,3,4,5.2))";
        let code = askql_parser::parse(ask_code.to_string(), false).unwrap();
        let result = vm.run(code, None, None).await;
        assert_eq!(Ok(Value::Float(-14.2)), result);
    }

    #[tokio::test]
    async fn chained_operation() {
        let vm = new_vm(vec![], HashMap::new());
        let ask_code = "ask(call(get('+'), call(get('-'),-2,3,4,5.2), call(get('+'),2,3,4,5.2)))";
        let code = askql_parser::parse(ask_code.to_string(), false).unwrap();
        let result = vm.run(code, None, None).await;
        assert_eq!(Ok(Value::Float(0.0)), result);
    }

    #[tokio::test]
    async fn complex_test() {
        let mut values = std::collections::HashMap::new();
        values.insert(
            "firstName".to_string(),
            AskCodeOrValue::new_value(Value::String("PrimeiroNome".to_string())),
        );
        values.insert(
            "lastName".to_string(),
            AskCodeOrValue::new_value(Value::String("SecondName".to_string())),
        );
        let mut friend0 = std::collections::BTreeMap::new();
        let mut friend1 = std::collections::BTreeMap::new();
        let mut friend2 = std::collections::BTreeMap::new();
        friend0.insert("id".to_string(), Value::Int(1));
        friend0.insert(
            "firstName".to_string(),
            Value::String("Friend 1".to_string()),
        );
        friend0.insert(
            "lastName".to_string(),
            Value::String("1".to_string())
        );
        friend1.insert("id".to_string(), Value::Int(2));
        friend1.insert(
            "firstName".to_string(),
            Value::String("Friend 2".to_string()),
        );
        friend1.insert(
            "lastName".to_string(),
            Value::String("2".to_string())
        );
        friend2.insert("id".to_string(), Value::Int(3));
        friend2.insert(
            "firstName".to_string(),
            Value::String("Friend 3".to_string()),
        );
        friend2.insert(
            "lastName".to_string(),
            Value::String("3".to_string())
        );
        let friends = vec![
            Value::Object(friend0),
            Value::Object(friend1),
            Value::Object(friend2),
        ];
        values.insert(
            "friends".to_string(),
            AskCodeOrValue::new_value(Value::List(friends)),
        );
        let vm = new_vm(vec![], values);
        let ask_code = "ask(query(node('firstName',f(call(get('concat'),call(get('toLowerCase'),get('firstName')),' ','is my ','name')))))";
        let code = askql_parser::parse(ask_code.to_string(), false).unwrap();
        let result = vm.run(code, None, None).await;
        let mut object_result = std::collections::BTreeMap::new();
        object_result.insert("firstName".to_string(), Value::String("primeironome is my name".to_string()));
        assert_eq!(Ok(Value::Object(object_result)), result);
    }
}

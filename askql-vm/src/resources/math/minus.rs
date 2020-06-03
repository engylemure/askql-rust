use crate::resource::Resource;
use crate::run::AskVm;
use askql_parser::{AskCode, AskCodeOrValue, Value};
use async_trait::async_trait;

pub struct MinusResource;

#[async_trait]
impl Resource for MinusResource {
    fn name(&self) -> String {
        "-".to_string()
    }
    async fn resolver(&self, args: Vec<Value>) -> Value {
        let (float_sum, int_sum, has_float, has_int) =
            args.into_iter()
                .enumerate()
                .fold((0.0, 0, false, false), |mut acc, (idx, val)| match val {
                    Value::Int(integer) => {
                        acc.3 = true;
                        if idx == 0 {
                            acc.1 = integer;
                        } else {
                            acc.1 -= integer;
                        };
                        acc
                    }
                    Value::Float(float) => {
                        acc.2 = true;
                        if idx == 0 {
                            acc.0 = float;
                        } else {
                            acc.0 -= float;
                        };
                        acc
                    }
                    _ => acc,
                });
        if has_float && has_int {
            Value::Float(float_sum + (int_sum as f32))
        } else if has_float {
            Value::Float(float_sum)
        } else {
            Value::Int(int_sum)
        }
    }
}

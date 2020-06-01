use crate::resource::Resource;
use crate::run::AskVm;
use askql_parser::{AskCode, AskCodeOrValue, Value};
use async_trait::async_trait;

pub struct SumResource;

#[async_trait]
impl Resource for SumResource {
    fn name(&self) -> String {
        "+".to_string()
    }
    async fn resolver(&self, args: Vec<Value>) -> Value {
        let (float_sum, int_sum) = args.into_iter()
        .fold((0.0, 0), |mut acc, val| {
            match val {
                Value::Int(integer) => {
                    acc.1 += integer;
                    acc
                },
                Value::Float(float) => {
                    acc.0 += float;
                    acc
                },
                _ => acc
            }
        });
        if float_sum != 0.0 && int_sum != 0 {
            Value::Float(float_sum + int_sum as f32)
        } else if float_sum != 0.0 {
            Value::Float(float_sum)
        } else {
            Value::Int(int_sum)
        }
    }
}

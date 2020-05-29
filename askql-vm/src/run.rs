use askql_parser::{AskCodeOrValue, is_ask_code};
use crate::r#type::Types;

pub async fn run(code: AskCodeOrValue) -> Result<Types, ()> {
    if !is_ask_code(code) {
        return 
    }
}
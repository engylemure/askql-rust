use crate::askcode::{AskCodeOrValue, AskCodeReducer};
use crate::reduce::{ParseError, Parser};

pub fn parse(code: String, logging: bool) -> Result<AskCodeOrValue, ParseError> {
    let mut parser = Parser::new(code, None, logging);
    let ask_code_reducer = AskCodeReducer {};
    parser.reduce(ask_code_reducer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::askcode::{Value, AskCode};
    use crate::reduce::ParseError;

    #[test]
    fn test_empty_program() {
        let repl = parse("".to_string(), true);
        assert!(repl.is_err());
        assert_eq!(repl.err().unwrap(), ParseError::EmptyProgram)
    }
    #[test]
    fn test_single_quote_string() {
        let repl = parse("'HelloWorld!'".to_string(), true);
        assert!(repl.is_ok());
        assert_eq!(repl.unwrap(), AskCodeOrValue::new_value(Value::String("HelloWorld!".to_string())));
    }

    #[test]
    fn test_double_quote_string() {
        let repl = parse(r#""HelloWorld!""#.to_string(), true);
        assert!(repl.is_ok());
        assert_eq!(repl.unwrap(), AskCodeOrValue::new_value(Value::String("HelloWorld!".to_string())));
    }

    #[test]
    fn test_int_number() {
        let repl = parse("4".to_string(), true);
        assert!(repl.is_ok());
        assert_eq!(repl.unwrap(), AskCodeOrValue::new_value(Value::Number("4".to_string())));
    }

    #[test]
    fn test_float_number() {
        let repl = parse("4.2".to_string(), true);
        assert!(repl.is_ok());
        assert_eq!(repl.unwrap(), AskCodeOrValue::new_value(Value::Number("4.2".to_string())));
    }

    #[test]
    fn test_expression() {
        let repl = parse("scorePerPhilosopher".to_string(), true);
        assert!(repl.is_ok());
        assert_eq!(repl.unwrap(), AskCodeOrValue::new_ask_code(AskCode::new("scorePerPhilosopher".to_string(), None)));
    }

    #[test]
    fn test_nested_expression() {
        let repl = parse("max(scorePerPhilosopher)".to_string(), true);
        assert!(repl.is_ok());
        let score_per_philosopher = AskCodeOrValue::new_ask_code(AskCode::new("scorePerPhilosopher".to_string(), None));
        let max = AskCodeOrValue::new_ask_code(AskCode::new("max".to_string(), Some(vec![score_per_philosopher])));
        assert_eq!(repl.unwrap(), max);
    }
}

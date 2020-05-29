use log::info;
use regex::Regex;

#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub enum Value {
    Null,
    Boolean(bool),
    Number(Number),
    String(String),
}

#[derive(Clone, Debug)]
pub struct Number(pub String);

pub trait Reducer<T> {
    fn node(&self, name: String, children: Option<Vec<T>>) -> T;
    fn id(&self, name: String) -> T;
    fn value(&self, value: Value) -> T;
}

impl Default for Value {
    fn default() -> Self {
        Value::Null
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ParseError {
    ExceedMaxStpes(usize),
    Expecting(char),
    Unknown(String),
    EmptyProgram,
}

pub struct Parser {
    code: Vec<char>,
    index: usize,
    steps: usize,
    id_regex: Regex,
    number_regex: Regex,
    stop_after_steps: Option<usize>,
    logging: bool,
}

impl Parser {
    pub fn new(code: String, stop_after_steps: Option<usize>, logging: bool) -> Self {
        Self {
            code: code.chars().collect(),
            index: 0,
            steps: 0,
            number_regex: Regex::new("[0-9.-]").unwrap(),
            id_regex: Regex::new("[_a-zA-Z0-9]").unwrap(),
            stop_after_steps,
            logging,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Delta {
    Subtract(usize),
    None,
}

impl Default for Delta {
    fn default() -> Self {
        Delta::None
    }
}

impl Parser {
    pub fn reduce<R, T>(&mut self, reducer: R) -> Result<T, ParseError>
    where
        R: Reducer<T>,
        T: std::fmt::Debug,
    {
        if self.code.is_empty() {
            return Err(ParseError::EmptyProgram)
        }
        self.program(&reducer)
    }
    fn delta(&self, delta: Delta) -> usize {
        use Delta::*;
        match delta {
            Subtract(d) => self.index.checked_sub(d).unwrap(),
            None => self.index,
        }
    }
    fn step(&mut self, message: String) -> Result<(), ParseError> {
        self.steps += 1;
        match self.stop_after_steps {
            Some(after_steps) if self.steps > after_steps => {
                Err(ParseError::ExceedMaxStpes(after_steps))
            }
            _ => {
                if self.logging {
                    info!("{}", message);
                    println!("{}", message);
                }
                Ok(())
            }
        }
    }

    fn is_at(&self, char: char, delta: Delta) -> bool {
        let pos = self.delta(delta);
        return pos < self.code.len() && self.code[pos] == char;
    }
    fn is_at_regex(&self, regex: &Regex, delta: Delta) -> bool {
        let pos = self.delta(delta);
        return pos < self.code.len() && regex.is_match(&self.code[pos].to_string());
    }
    fn process(&mut self, char: char) -> Result<usize, ParseError> {
        if !self.is_at(char, Delta::None) {
            return Err(ParseError::Expecting(char));
        }
        self.step(format!("process({})", char))?;
        self.index += 1;
        Ok(self.index)
    }
    fn whitespace(&mut self) {
        while self.is_at(' ', Delta::None) || self.is_at('\n', Delta::None) {
            self.index += 1
        }
    }
    fn id(&mut self) -> Result<&[char], ParseError> {
        self.whitespace();
        self.step("id".to_string())?;
        let start = self.index;
        while self.index < self.code.len() && self.is_at_regex(&self.id_regex, Delta::None) {
            self.index += 1;
        }
        if self.index == start {
            return Err(ParseError::Unknown(format!("id error at index: {}", self.index)));
        }
        return Ok(self.code.get(start..self.index).unwrap());
    }
    fn program<U, R>(&mut self, reducer: &R) -> Result<U, ParseError>
    where
        R: Reducer<U>,
        U: std::fmt::Debug,
    {
        let value = self.expression(reducer);
        self.whitespace();
        if self.index < self.code.len() {
            return Err(ParseError::Unknown(format!("program error at index: {}", self.index)));
        }
        value
    }

    fn expression<U, R>(&mut self, reducer: &R) -> Result<U, ParseError>
    where
        R: Reducer<U>,
        U: std::fmt::Debug,
    {
        self.whitespace();
        self.step("expression".to_string())?;
        if self.is_at('"', Delta::None) || self.is_at('\'', Delta::None) {
            return self.string(reducer);
        }
        if self.is_at_regex(&self.number_regex, Delta::None) {
            return self.number(reducer);
        }
        if self.is_at('[', Delta::None) {
            return self.expression_list(reducer, "list".to_string(), '[', ']', ',', ',');
        }
        if self.is_at('{', Delta::None) {
            return self.expression_list(reducer, "object".to_string(), '{', '}', ',', ':');
        }
        return self.call(reducer);
    }

    fn expression_list<U, R>(
        &mut self,
        reducer: &R,
        name: String,
        open_char: char,
        close_char: char,
        separator: char,
        odd_separator: char,
    ) -> Result<U, ParseError>
    where
        R: Reducer<U>,
        U: std::fmt::Debug,
    {
        self.process(open_char)?;
        self.whitespace();
        let mut values: Vec<U> = Vec::new();
        let start = self.index;
        while self.index < self.code.len() && !self.is_at(close_char, Delta::None) {
            let value = self.expression(reducer)?;
            self.step(format!("list item {:?}", value))?;
            values.push(value);
            self.whitespace();
            if !self.is_at(close_char, Delta::None) {
                self.process(if (self.index - start) % 2 == 0 {
                    odd_separator
                } else {
                    separator
                })?;
            }
        }
        self.process(close_char)?;
        Ok(reducer.node(name, Some(values)))
    }

    fn number<U, R>(&mut self, reducer: &R) -> Result<U, ParseError>
    where
        R: Reducer<U>,
        U: std::fmt::Debug,
    {
        self.step("number".to_string())?;
        let start = self.index;
        while self.index < self.code.len() && self.is_at_regex(&self.number_regex, Delta::None) {
            self.index += 1;
        }
        let end = self.index;
        Ok(reducer.value(Value::Number(Number(
            self.code.get(start..end).unwrap().iter().collect(),
        ))))
    }

    fn string<U, R>(&mut self, reducer: &R) -> Result<U, ParseError>
    where
        R: Reducer<U>,
        U: std::fmt::Debug,
    {
        self.step("string".to_string())?;
        let quote = self.code[self.index];
        self.process(quote)?;
        let start = self.index;
        while self.index < self.code.len()
            && !(!self.is_at('\\', Delta::Subtract(1)) && self.is_at(quote, Delta::None))
        {
            self.index += 1;
        }
        if self.index == self.code.len() {
            return Err(ParseError::Unknown(format!("string error at index: {}", self.index)));
        }
        let end = self.index;
        self.process(quote)?;
        Ok(reducer.value(Value::String(
            self.code.get(start..end).expect("Error in String").iter().collect(),
        )))
    }

    fn call<U, R>(&mut self, reducer: &R) -> Result<U, ParseError>
    where
        R: Reducer<U>,
        U: std::fmt::Debug,
    {
        self.whitespace();
        let name: String = self.id()?.iter().collect();
        self.whitespace();
        if !self.is_at('(', Delta::None) {
            return Ok(reducer.id(name));
        }
        self.whitespace();
        self.expression_list(reducer, name, '(', ')', ',', ',')
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

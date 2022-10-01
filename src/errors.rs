use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct ParserError {
    message: String,
}

impl Error for ParserError {}
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParserError: {}", self.message)
    }
}
impl ParserError {
    pub fn new(message: &str) -> ParserError {
        ParserError {
            message: message.to_string(),
        }
    }
}
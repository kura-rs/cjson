use crate::Value;

/// An error returned when the input is not valid JSON.
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// The input does not start with any known JSON value.
    InvalidValue,
}

/// Holds the input and the current reading position.
///
/// Corresponds to 'parse_buffer' in cJSON.c:
/// 'bytes' is 'content', and 'pos' is 'offset'.
struct Parser {
    bytes: Vec<u8>,
    pos: usize,
}

impl Parser {
    fn new(input: &str) -> Parser {
        Parser {
            bytes: input.as_bytes().to_vec(),
            pos: 0,
        }
    }

    /// If the unread part starts with 'literal', moves past it and returns true.
    ///
    /// Replaces 'can_read' + 'strncmp' + 'offset += n' in cJSON.c.
    fn consume(&mut self, literal: &str) -> bool {
        let rest = &self.bytes[self.pos..];
        if rest.starts_with(literal.as_bytes()) {
            self.pos += literal.len();
            true
        } else {
            false
        }
    }

    /// Returns the next unread byte without moving past it.
    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    /// Consumes 'word' and returns 'value', or fails if the input does not match.
    fn literal(&mut self, word: &str, value: Value) -> Result<Value, ParseError> {
        if self.consume(word) {
            Ok(value)
        } else {
            Err(ParseError::InvalidValue)
        }
    }

    /// Corresponds to 'parse_value' in cJSON.c.
    /// Looks at the first byte to decide which kind of value comes next.
    fn parse_value(&mut self) -> Result<Value, ParseError> {
        match self.peek() {
            Some(b'n') => self.literal("null", Value::Null),
            Some(b'f') => self.literal("false", Value::Bool(false)),
            Some(b't') => self.literal("true", Value::Bool(true)),
            _ => Err(ParseError::InvalidValue),
        }
    }
}

/// Parses a JSON text into a 'Value'.
///
/// Corresponds to 'cJSON_Parse' in cJSON.h.
pub fn parse(input: &str) -> Result<Value, ParseError> {
    let mut parser = Parser::new(input);
    parser.parse_value()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_literals() {
        assert_eq!(parse("null"), Ok(Value::Null));
        assert_eq!(parse("true"), Ok(Value::Bool(true)));
        assert_eq!(parse("false"), Ok(Value::Bool(false)));
    }

    #[test]
    fn reject_unknown() {
        assert_eq!(parse("nul"), Err(ParseError::InvalidValue));
        assert_eq!(parse("hello"), Err(ParseError::InvalidValue));
        assert_eq!(parse(""), Err(ParseError::InvalidValue));
    }
}

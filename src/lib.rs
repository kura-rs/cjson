#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(Vec<(String, Value)>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_numbers_are_equal() {
        let a = Value::Number(1.0);
        let b = Value::Number(1.0);
        assert_eq!(a, b);
    }

    #[test]
    fn different_values_are_equal() {
        let a = Value::Null;
        let b = Value::Bool(false);
        assert_ne!(a, b);
    }

    #[test]
    fn nested_values_are_equal() {
        let name = Value::String("kura".to_string());
        let pair = ("name".to_string(), name);
        let object = Value::Object(vec![pair]);
        let expected = Value::Array(vec![object]);

        let name = Value::String("kura".to_string());
        let pair = ("name".to_string(), name);
        let object = Value::Object(vec![pair]);
        let actual = Value::Array(vec![object]);

        assert_eq!(expected, actual);
    }
}

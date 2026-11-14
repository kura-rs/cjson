/// A JSON value.
///
/// Corresponds to `struct cJSON` in cJSON.h. cJSON stores every kind of
/// value in one struct and uses the `type` field to tell them apart;
/// here each variant holds only the data it needs.
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
    fn build_values() {
        let b = Value::Bool(true);
        assert_eq!(b, Value::Bool(true));

        let arr = Value::Array(vec![Value::Number(1.0), Value::Null]);
        assert_eq!(arr, Value::Array(vec![Value::Number(1.0), Value::Null]));

        let obj = Value::Object(vec![
            ("name".to_string(), Value::String("kura".to_string())),
        ]);
        assert_ne!(obj, Value::Null);
    }
}

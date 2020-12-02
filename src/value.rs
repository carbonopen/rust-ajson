use getter::Getter;
use number::Number;
use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::str;

/// Represents JSON valuue.
#[derive(PartialEq, Clone)]
pub enum Value {
    /// Represents a JSON String.
    String(String),
    /// Respesents a JSON number.
    Number(Number),
    /// Respesents a JSON object.
    Object(String),
    /// Respesents a JSON array.
    Array(String),
    /// Respesents a JSON boolean.
    Boolean(bool),
    /// Respesents a JSON null value.
    Null,
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(_) => write!(f, r#""{}""#, self.as_str()),
            _ => write!(f, "{}", self.as_str()),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Value {
    /// Get sub value from a JSON array or map.
    /// About path syntax, see [here](index.html#syntax).
    /// For more detail, see [`get`](fn.get.html).
    /// ```
    /// use ajson::Value;
    ///
    /// let v = Value::Array("[1,2,3]".to_owned());
    /// let first_num = v.get("0").unwrap();
    /// assert_eq!(first_num.to_i64(), 1_i64);
    /// ```
    pub fn get(&self, path: &str) -> Option<Value> {
        self.get_by_utf8(&path.as_bytes())
    }

    #[doc(hidden)]
    pub fn get_by_utf8(&self, v: &[u8]) -> Option<Value> {
        match self {
            Value::Array(s) | Value::Object(s) => Getter::from_str(s).get_by_utf8(v),
            _ => None,
        }
    }
}

impl Value {

    /// Returns true if the `Value` is a JSON string.
    /// ```
    /// let v = ajson::get(r#"{"name":"ajson"}"#, "name").unwrap();
    /// assert!(v.is_string());
    /// ```
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Value::Array(_))
    }

    pub fn is_object(&self) -> bool {
        matches!(self, Value::Object(_))
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Boolean(_))
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }
}

impl Value {
    pub fn as_str(&self) -> &str {
        match &self {
            Value::String(ref s) => s,
            Value::Number(number) => number.as_str(),
            Value::Boolean(true) => "true",
            Value::Boolean(false) => "false",
            Value::Object(ref s) => s,
            Value::Array(ref s) => s,
            Value::Null => "null",
        }
    }

    pub fn to_f64(&self) -> f64 {
        match self {
            Value::Number(number) => number.to_f64(),
            Value::Boolean(true) => 1.0,
            Value::String(s) => Number::from(s.as_bytes()).to_f64(),
            _ => 0.0,
        }
    }

    pub fn to_u64(&self) -> u64 {
        match self {
            Value::Number(number) => number.to_u64(),
            Value::Boolean(true) => 1,
            Value::String(s) => Number::from(s.as_bytes()).to_u64(),
            _ => 0,
        }
    }

    pub fn to_i64(&self) -> i64 {
        match self {
            Value::Number(number) => number.to_i64(),
            Value::Boolean(true) => 1,
            Value::String(ref s) => Number::from(s.as_bytes()).to_i64(),
            _ => 0,
        }
    }

    pub fn to_bool(&self) -> bool {
        match *self {
            Value::Boolean(b) => b,
            _ => false,
        }
    }

    pub fn to_vec(&self) -> Vec<Value> {
        match self {
            Value::Array(s) => Getter::from_str(s).to_vec(),
            Value::Null => vec![],
            _ => vec![self.clone()],
        }
    }

    pub fn to_object(&self) -> HashMap<String, Value> {
        match self {
            Value::Object(s) => Getter::from_str(s).to_object(),
            _ => HashMap::new(),
        }
    }
}

impl<'a> cmp::PartialEq<&'a str> for Value {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl cmp::PartialEq<f64> for Value {
    fn eq(&self, other: &f64) -> bool {
        self.to_f64() == *other
    }
}

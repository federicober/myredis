#[derive(Debug, PartialEq)]
pub enum TypedValue {
    Numeric(f64),
    Text(String),
}

impl TypedValue {
    pub fn from_str(as_str: &str) -> Self {
        match as_str.parse::<f64>() {
            Ok(num) => TypedValue::Numeric(num),
            Err(_) => TypedValue::Text(String::from(as_str)),
        }
    }
}

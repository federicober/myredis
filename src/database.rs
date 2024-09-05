use crate::types::TypedValue;

use super::types;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Database {
    data: HashMap<String, types::TypedValue>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    pub fn set(&mut self, key: &str, value: TypedValue) {
        self.data.insert(key.to_string(), value);
    }
    pub fn get(&mut self, key: &str) -> Option<&types::TypedValue> {
        self.data.get(&key.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_delete() {
        let mut db = Database::new();
        db.set("foo", types::TypedValue::Text(String::from("bar")));
        assert_eq!(
            db.get("foo"),
            Some(&types::TypedValue::Text(String::from("bar")))
        )
    }
}

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
    pub fn set(&mut self, key: String, value: TypedValue) {
        self.data.insert(key, value);
    }
    pub fn get(&mut self, key: String) -> Option<&types::TypedValue> {
        self.data.get(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_delete() {
        let mut db = Database::new();
        db.set(
            String::from("foo"),
            types::TypedValue::Text(String::from("bar")),
        );
        assert_eq!(
            db.get(String::from("foo")),
            Some(&types::TypedValue::Text(String::from("bar")))
        )
    }
}

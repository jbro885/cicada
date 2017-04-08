use std::collections::HashMap;

pub struct Shell {
    pub alias: HashMap<String, String>,
}

impl Shell {
    pub fn new() -> Shell {
        Shell {
            alias: HashMap::new(),
        }
    }

    pub fn add_alias(&mut self, name: &str, value: &str) {
        self.alias.insert(name.to_string(), value.to_string());
    }

    pub fn extend_alias(&mut self, name: &str) -> String {
        let result;
        match self.alias.get(name) {
            Some(x) => {
                result = x.to_string();
            }
            None => {
                result = name.to_string();
            }
        }
        return result;
    }
}
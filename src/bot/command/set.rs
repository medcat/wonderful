use super::Command;
use std::collections::HashMap;

pub struct Set(HashMap<&'static str, &'static Command<'static>>);

impl Set {
    pub fn new() -> Set { Set(HashMap::new()) }
    pub fn insert(&mut self, command: &'static Command) {
        self.0.insert(&command.name, command);
        for alias in command.aliases { self.0.insert(&alias, command); }
    }

    pub fn contains_key(&self, key: &str) -> bool { self.0.contains_key(key) }
}

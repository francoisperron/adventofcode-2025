use std::collections::HashMap;

#[derive(Default)]
pub struct Cache(HashMap<(String, bool, bool), usize>);

impl Cache {
    pub fn put(&mut self, key: (String, bool, bool), value: usize) {
        self.0.insert(key, value);
    }

    pub fn get(&self, key: &(String, bool, bool)) -> Option<&usize> {
        self.0.get(key)
    }
}

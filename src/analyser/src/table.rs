use std::collections::HashMap;

// Define the HashMap structure
pub struct ScopeTable<T>
{
    pub symbols: HashMap<String, T>
}

impl<T> ScopeTable<T>
{
    pub fn new() -> Self {
        ScopeTable {
            symbols: HashMap::new()
        }
    }

    pub fn insert(&mut self, key: String, value: T){
        self.symbols.insert(key, value);
    }

    pub fn lookup(&self, key: &String) -> Option<&T> {
        self.symbols.get(key)
    }

    pub fn get_mut(&mut self, key: &String) -> Option<&mut T> {
        self.symbols.get_mut(key)
    }

    pub fn remove(&mut self, key: &String) -> Option<T> {
        self.symbols.remove(key)
    }

    pub fn is_empty(&self) -> bool {
        self.symbols.is_empty()
    }

    pub fn update(&mut self, key: String, value: T) {
        self.symbols.insert(key, value);
    }
}
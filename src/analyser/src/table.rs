use std::hash::{Hash, Hasher};
use std::collections::{HashMap, LinkedList};
use std::collections::hash_map::DefaultHasher;
use crate::symbol::Symbol;
use spl_ast::tree::Value;
use crate::symbol::Type;

// Define the HashMap structure
pub struct HashLinkedMap<T> {
    symbols: HashMap<String, LinkedList<Symbol<T>>>,
    variable_count: i32,
}

impl<T> HashLinkedMap<T>
{
    pub fn new() -> Self {
        HashLinkedMap {
            symbols: HashMap::new(),
            variable_count: 0,
        }
    }

    fn insert(&mut self, symbol: Symbol<T>) {
        let key = symbol.identifier.clone();
        let dummy_symbol = Symbol {
            id: -1,
            is_global: false,
            identifier: "".to_string(),
            symbol_type: None,
            symbol_table_next: None,
            scope_stack_next: None,
        };
        let symbol_list = self.symbols.entry(key).or_insert(
            LinkedList::from(dummy_symbol)
        );
        symbol_list.push_back(symbol);
    }

    fn get(&self, ident: String) -> Option<&Symbol<T>> {
        
    }

    // Remove a key-value pair
    fn remove(&mut self, key: &K) -> Option<V> {
        let index = self.get_bucket_index(key);
        let bucket = &mut self.buckets[index];

        // Find and remove the key-value pair
        let position = bucket.iter().position(|pair| &pair.key == key)?;
        let removed = bucket.remove(position)?;
        Some(removed.value)
    }
}

impl<Symbol<T>> LinkedList<Symbol<T>> {
    fn push_back(&mut self, symbol: Symbol<T>) {
        let mut current = self.head.take();
        let new_node = Box::new(Node {
            data: symbol,
            next: current,
        });
        self.head = Some(new_node);
    }
}
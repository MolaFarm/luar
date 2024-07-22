use std::collections::HashMap;
use std::hash::Hash;

pub struct ValueHashMap<K, V> {
    map: HashMap<K, V>,
}

impl<K: Eq + Hash + Clone, V: Clone> ValueHashMap<K, V> {
    pub fn new() -> Self {
        ValueHashMap {
            map: HashMap::new(),
        }
    }

    pub fn get_value(&mut self, key: K) -> Option<V> {
        self.map.get(&key).cloned()
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }
}
use std::collections::HashMap;
use std::hash::Hash;

pub trait Compute<K, V> {

    fn compute<F>(&mut self, key: K, f: F) -> Option<V> where F: FnOnce(Option<&V>) -> V, K: Eq + Hash;

}

impl <K, V> Compute<K, V> for HashMap<K, V> {

    fn compute<F>(&mut self, key: K, f: F) -> Option<V> where F: FnOnce(Option<&V>) -> V, K: Eq + Hash {
        let new_value = f(self.get(&key));
        self.insert(key, new_value)
    }

}
#![feature(test)]

extern crate test;

pub mod idx {
    use std::collections::HashMap;
    use std::hash::Hash;
    use std::mem;

    pub type Idx = usize;

    pub struct MultiMap<K, V>
    where
        K: Eq + Hash,
    {
        main: HashMap<K, Idx>,
        refs: HashMap<Idx, V>,
        counter: Idx,
    }

    impl<K, V> MultiMap<K, V>
    where
        K: Eq + Hash,
    {
        pub fn new() -> MultiMap<K, V> {
            MultiMap {
                main: HashMap::new(),
                refs: HashMap::new(),
                counter: 0,
            }
        }

        pub fn get(&self, key: &K) -> Option<&V> {
            self.main
                .get(key)
                .and_then(|reference| self.refs.get(reference))
        }

        pub fn new_v(&mut self, value: V) -> Idx {
            let new = self.counter + 1;
            let idx = mem::replace(&mut self.counter, new);

            self.refs.insert(idx, value);

            idx
        }

        pub fn insert(&mut self, k: K, index: Idx) {
            self.main.insert(k, index);
        }
    }
}

pub mod rc {
    use std::collections::HashMap;
    use std::hash::Hash;
    use std::rc::Rc;

    pub type MultiMap<K, V> = HashMap<K, Rc<V>>;

    pub trait Ext<K, V> {
        fn new_v(&mut self, value: V) -> Rc<V>;

        fn insert_r(&mut self, k: K, idx: &Rc<V>);
    }

    impl<K, V> Ext<K, V> for MultiMap<K, V>
    where
        K: Eq + Hash,
    {
        fn new_v(&mut self, value: V) -> Rc<V> {
            Rc::new(value)
        }

        fn insert_r(&mut self, k: K, idx: &Rc<V>) {
            self.insert(k, idx.clone());
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use test::bench::Bencher;
    use test::black_box;

    #[bench]
    fn idx_multimap(b: &mut Bencher) {
        let mut map: idx::MultiMap<String, String> = idx::MultiMap::new();

        b.iter(|| {
            let idx = map.new_v(String::from("value"));

            for _ in 0..4000 {
                map.insert(String::from("key"), idx);
            }
        });
    }

    #[bench]
    fn rc_multimap(b: &mut Bencher) {
        use rc::Ext;

        let mut map: rc::MultiMap<String, String> = rc::MultiMap::new();

        b.iter(|| {
            let idx = map.new_v(String::from("value"));

            for _ in 0..4000 {
                map.insert_r(String::from("key"), &idx);
            }
        });
    }
}

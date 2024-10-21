use std::borrow::Borrow;
use std::cmp::max;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::mem::{swap, take};

// source of inspiration:  https://medium.com/@techhara/rust-implementation-challenge-hash-table-356def8e31ce
// for collision resolution, I used linear probing open addressing

enum Entry<Key, Value> {
    Vacant,
    Occupied { key: Key, value: Value },
}
impl<Key, Value> Entry<Key, Value> {
    fn take(&mut self) -> Option<Value> {
        match self {
            Self::Occupied { key: _, value: _ } => {
                let mut occupied = Self::Vacant;
                swap(self, &mut occupied);
                if let Self::Occupied { key: _, value } = occupied {
                    Some(value)
                } else {
                    panic!("fatal: unreachable");
                }
            }
            _ => None,
        }
    }

    fn replace(&mut self, mut x: Value) -> Option<Value> {
        match self {
            Self::Occupied { key: _, value } => {
                swap(&mut x, value);
                Some(x)
            }
            _ => None,
        }
    }
}

pub struct HashTable<Key, Value> {
    entries: Vec<Entry<Key, Value>>,
    occupied_size: usize,
    vacant_size: usize,
}

impl<Key: Eq + Hash, Value> HashTable<Key, Value> {
    const ACCEPTABLE_LOAD_FACTOR: f64 = 0.75;
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            occupied_size: 0,
            vacant_size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.occupied_size
    }
    fn get_index<Q>(&self, key: &Q) -> usize
    where
        Key: Borrow<Q>,
        Q: Eq + Hash,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.entries.len()
    }
    fn iter_mut_starting_at(
        &mut self,
        index: usize,
    ) -> impl Iterator<Item = &mut Entry<Key, Value>> {
        let (s1, s2) = self.entries.split_at_mut(index);
        s2.iter_mut().chain(s1.iter_mut())
    }

    fn get_load_factor(&self) -> f64 {
        if self.entries.is_empty() {
            1.0
        } else {
            self.occupied_size as f64 / self.entries.len() as f64
        }
    }

    fn resize(&mut self) {
        let resize_factor = if self.get_load_factor() > Self::ACCEPTABLE_LOAD_FACTOR { 2 } else { 1 };
        let new_size = max(64, self.entries.len() * resize_factor);
        let mut new_entries = Self {
            entries: (0..new_size).map(|_| Entry::Vacant).collect(),
            occupied_size: 0,
            vacant_size: new_size,
        };
        for entry in take(&mut self.entries) {
            if let Entry::Occupied { key, value } = entry {
                new_entries.insert_without_resize(key, value);
            }
        }
        swap(self, &mut new_entries)
    }

    fn insert_without_resize(&mut self, key: Key, value: Value) -> Option<Value> {
        let index = self.get_index(&key);
        let mut result = None;
        for entry in self.iter_mut_starting_at(index) {
            match entry {
                Entry::Occupied { key: k, .. } if (k as &Key).borrow() == &key => {
                    result = entry.replace(value);
                    break;
                }
                Entry::Vacant => {
                    *entry = Entry::Occupied { key, value };
                    break;
                }
                _ => {}
            }
        }
        if result.is_none() {
            self.occupied_size += 1;
            self.vacant_size -= 1;
        }
        result
    }

    pub fn insert(&mut self, key: Key, value: Value) {
        if self.get_load_factor() >= Self::ACCEPTABLE_LOAD_FACTOR {
            self.resize();
        }
        self.insert_without_resize(key, value);
    }
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        Key: Borrow<Q>,
        Q: Eq + Hash,
    {
        self.get(key).is_some()
    }
    
    pub fn iter(&self) -> impl Iterator<Item = (&Key, &Value)> {
        self.entries
            .iter()
            .filter_map(|entry| match entry {
                Entry::Occupied { key, value } => Some((key, value)),
                _ => None,
            })
    }
    
    pub fn get<Q>(&self, key: &Q) -> Option<&Value>
    where
        Key: Borrow<Q>,
        Q: Eq + Hash,
    {
        if self.len() == 0 {
            return None;
        }
        let mut key_index = self.get_index(key);
        loop {
            match &self.entries[key_index] {
                Entry::Vacant => {
                    break None;
                }
                Entry::Occupied { key: k, value: v } if k.borrow() == key => {
                    break Some(v);
                }
                _ => {
                    key_index = (key_index + 1) % self.entries.len();
                }
            }
        }
    }
    

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut Value>
    where
        Key: Borrow<Q>,
        Q: Eq + Hash,
    {
        if self.len() == 0 {
            return None;
        }
        let key_index = self.get_index(key);
        for entry in self.iter_mut_starting_at(key_index) {
            match entry {
                Entry::Vacant => {
                    return None;
                }
                Entry::Occupied { key: k, value: v } if (k as &Key).borrow() == key => {
                    return Some(v);
                }
                _ => {}
            }
        }
        panic!("fatal: unreachable");
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<Value>
    where
        Key: Borrow<Q>,
        Q: Eq + Hash,
    {
        if self.len() == 0 {
            return None;
        }
        let key_index = self.get_index(key);
        let mut result = None;
        for entry in self.iter_mut_starting_at(key_index) {
            match entry {
                Entry::Occupied { key: k, .. } if (k as &Key).borrow() == key => {
                    result = entry.take();
                    break;
                }
                Entry::Vacant => {
                    result = None;
                    break;
                }
                _ => {}
            }
        }
        result.map(|val| {
            self.occupied_size -= 1;
            val
        })
    }
}

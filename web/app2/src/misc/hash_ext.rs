use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub trait Hashable {
    fn hashed(&self) -> u64;
}

impl<T: Hash> Hashable for T {
    fn hashed(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

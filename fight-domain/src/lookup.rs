use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Deref, DerefMut};

use indexmap::IndexMap;
pub use indexmap::map::{IntoValues, Values, ValuesMut};
use serde::{Deserialize, Serialize};

pub trait LookupKey: Hash {
    type Key: Clone + Hash + Eq + Debug;
    fn lookup_key(&self) -> &Self::Key;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(bound(
    serialize = "V: Serialize, V::Key: Serialize",
    deserialize = "V: Deserialize<'de>, V::Key: Deserialize<'de>"
))]
pub struct Lookup<V>
where
    V: LookupKey,
{
    container: IndexMap<V::Key, V>,
    #[serde(skip)]
    hasher: DefaultHasher,
    hash: u64,
}

impl<V> Lookup<V>
where
    V: LookupKey,
{
    pub fn get(&self, key: &V::Key) -> Option<&V> {
        self.container
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| v)
    }

    pub fn get_mut(&mut self, key: V::Key) -> Option<WriteGuard<'_, V>> {
        self.container
            .iter_mut()
            .find(|(k, _)| **k == key)
            .map(|(_, v)| WriteGuard::new(v, &mut self.hasher, &mut self.hash))
    }

    pub fn put(&mut self, value: V) {
        value.hash(&mut self.hasher);
        self.hash = self.hasher.finish();
        self.container.insert(value.lookup_key().clone(), value);
    }

    pub fn take(&mut self, key: &V::Key) -> Option<V> {
        None::<V>.hash(&mut self.hasher);
        self.hash = self.hasher.finish();
        self.container.shift_remove(key)
    }

    pub fn replace(&mut self, key: &V::Key, value: V) {
        value.hash(&mut self.hasher);
        self.hash = self.hasher.finish();
        let prev = self.container.insert(key.clone(), value);
        if prev.is_none() {
            panic!("{:?} not found in Lookup", key)
        }
    }

    pub fn insert(&mut self, index: usize, value: V) {
        value.hash(&mut self.hasher);
        self.hash = self.hasher.finish();
        self.container
            .shift_insert(index, value.lookup_key().clone(), value);
    }

    pub fn len(&self) -> usize {
        self.container.len()
    }

    pub fn is_empty(&self) -> bool {
        self.container.is_empty()
    }

    pub fn contains_key(&self, key: &V::Key) -> bool {
        self.container.contains_key(key)
    }

    pub fn contains_value(&self, value: &V) -> bool {
        let key = value.lookup_key();
        self.contains_key(key)
    }

    pub fn iter(&self) -> Values<'_, V::Key, V> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> ValuesMut<'_, V::Key, V> {
        self.into_iter()
    }
}

impl<V> Default for Lookup<V>
where
    V: LookupKey,
{
    fn default() -> Self {
        Self {
            container: Default::default(),
            hasher: Default::default(),
            hash: 0,
        }
    }
}

impl<V> PartialEq for Lookup<V>
where
    V: LookupKey + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash && self.container == other.container
    }
}

impl<V> Hash for Lookup<V>
where
    V: LookupKey + Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        for value in self.container.values() {
            value.hash(state);
        }
        self.hash.hash(state);
    }
}

impl<V> Eq for Lookup<V> where V: LookupKey + PartialEq {}

impl<V> Extend<V> for Lookup<V>
where
    V: LookupKey,
{
    fn extend<T: IntoIterator<Item = V>>(&mut self, iter: T) {
        for value in iter {
            self.put(value)
        }
    }
}

impl<V> FromIterator<V> for Lookup<V>
where
    V: LookupKey,
{
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        let container = IndexMap::from_iter(iter.into_iter().map(|v| (v.lookup_key().clone(), v)));
        let mut hasher = DefaultHasher::new();
        for value in container.values() {
            value.hash(&mut hasher);
        }
        let hash = hasher.finish();
        Self {
            container,
            hasher,
            hash,
        }
    }
}

impl<V> IntoIterator for Lookup<V>
where
    V: LookupKey,
{
    type Item = V;
    type IntoIter = IntoValues<V::Key, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.container.into_values()
    }
}

impl<'a, V> IntoIterator for &'a Lookup<V>
where
    V: LookupKey,
{
    type Item = &'a V;
    type IntoIter = Values<'a, V::Key, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.container.values()
    }
}

impl<'a, V> IntoIterator for &'a mut Lookup<V>
where
    V: LookupKey,
{
    type Item = &'a mut V;
    type IntoIter = ValuesMut<'a, V::Key, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.container.values_mut()
    }
}

#[derive(Debug)]
pub struct WriteGuard<'a, V>
where
    V: LookupKey,
{
    value: &'a mut V,
    hasher: &'a mut DefaultHasher,
    hash: &'a mut u64,
}

impl<'a, V> WriteGuard<'a, V>
where
    V: LookupKey,
{
    fn new(value: &'a mut V, hasher: &'a mut DefaultHasher, hash: &'a mut u64) -> Self {
        Self {
            value,
            hasher,
            hash,
        }
    }
}

impl<'a, V> Drop for WriteGuard<'a, V>
where
    V: LookupKey,
{
    fn drop(&mut self) {
        self.value.hash(&mut self.hasher);
        *self.hash = self.hasher.finish();
    }
}

impl<'a, V> Deref for WriteGuard<'a, V>
where
    V: LookupKey,
{
    type Target = V;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'a, V> DerefMut for WriteGuard<'a, V>
where
    V: LookupKey,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value
    }
}

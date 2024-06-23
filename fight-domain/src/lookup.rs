use std::fmt::Debug;
use std::hash::Hash;

use indexmap::IndexMap;
pub use indexmap::map::{IntoValues, Values, ValuesMut};
use serde::{Deserialize, Serialize};

pub trait LookupKey {
    type Key: Clone + Hash + Eq + Debug;
    fn lookup_key(&self) -> Self::Key;
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(bound(
    serialize = "V: Serialize, V::Key: Serialize",
    deserialize = "V: Deserialize<'de>, V::Key: Deserialize<'de>"
))]
pub struct Lookup<V>
where
    V: LookupKey,
{
    container: IndexMap<V::Key, V>,
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

    pub fn get_mut(&mut self, key: &V::Key) -> Option<&mut V> {
        self.container
            .iter_mut()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| v)
    }

    pub fn put(&mut self, value: V) {
        self.container.insert(value.lookup_key().clone(), value);
    }

    pub fn take(&mut self, key: &V::Key) -> Option<V> {
        self.container.shift_remove(key)
    }

    pub fn replace(&mut self, old_key: &V::Key, new_value: V) {
        if !self.container.contains_key(old_key) {
            panic!("{:?} not found in Lookup", old_key)
        }
        let new_key = new_value.lookup_key();
        if self.container.contains_key(&new_key) {
            panic!("{:?} already exists in Lookup", new_key)
        }
        // put the value in the last position
        self.container.insert(new_key, new_value);
        // remove the old value, which swaps in the last (just inserted) value
        self.container.swap_remove(old_key);
    }

    pub fn insert(&mut self, index: usize, value: V) {
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
        self.contains_key(&key)
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
        }
    }
}

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
        Self {
            container: IndexMap::from_iter(iter.into_iter().map(|v| (v.lookup_key().clone(), v))),
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

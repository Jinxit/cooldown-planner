use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub trait LookupKey {
    type Key: Clone + PartialEq + Debug;
    fn lookup_key(&self) -> &Self::Key;
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(bound(
    serialize = "V: Serialize, V::Key: Serialize",
    deserialize = "V: Deserialize<'de>, V::Key: Deserialize<'de>"
))]
pub struct Lookup<V>
where
    V: LookupKey,
{
    container: Vec<(V::Key, V)>,
}

impl<V> Lookup<V>
where
    V: LookupKey,
{
    pub fn get(&self, key: &V::Key) -> Option<&V> {
        self.container
            .iter()
            .find(|(k, _)| *k == *key)
            .map(|(_, v)| v)
    }

    pub fn get_mut(&mut self, key: &V::Key) -> Option<&mut V> {
        self.container
            .iter_mut()
            .find(|(k, _)| *k == *key)
            .map(|(_, v)| v)
    }

    pub fn put(&mut self, value: V) {
        if let Some(existing) = self
            .container
            .iter_mut()
            .find(|(k, _)| k == value.lookup_key())
        {
            existing.1 = value;
        } else {
            self.container.push((value.lookup_key().clone(), value))
        }
    }

    pub fn take(&mut self, key: &V::Key) -> Option<V> {
        let index = self.container.iter().position(|(k, _)| *k == *key);
        index
            .map(|index| self.container.remove(index))
            .map(|(_, v)| v)
    }

    pub fn replace(&mut self, key: &V::Key, value: V) {
        let index = self
            .container
            .iter()
            .position(|(k, _)| *k == *key)
            .unwrap_or_else(|| panic!("{:?} not found in Lookup", key));
        *self.container.get_mut(index).unwrap() = (value.lookup_key().clone(), value);
    }

    pub fn insert(&mut self, index: usize, value: V) {
        self.container
            .insert(index, (value.lookup_key().clone(), value));
    }

    pub fn len(&self) -> usize {
        self.container.len()
    }

    pub fn is_empty(&self) -> bool {
        self.container.is_empty()
    }

    pub fn contains_key(&self, key: &V::Key) -> bool {
        self.container.iter().any(|(k, _)| *k == *key)
    }

    pub fn contains_value(&self, value: &V) -> bool {
        let key = value.lookup_key();
        self.contains_key(key)
    }

    pub fn iter(&self) -> Iter<'_, V> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.container.iter_mut().map(|(_, v)| v)
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
            container: Vec::from_iter(iter.into_iter().map(|v| (v.lookup_key().clone(), v))),
        }
    }
}

#[derive(Clone)]
pub struct IntoIter<V>
where
    V: LookupKey,
{
    lookup: Lookup<V>,
}

impl<V> IntoIterator for Lookup<V>
where
    V: LookupKey,
{
    type Item = V;
    type IntoIter = IntoIter<V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { lookup: self }
    }
}

impl<V> Iterator for IntoIter<V>
where
    V: LookupKey,
{
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        if self.lookup.container.is_empty() {
            None
        } else {
            Some(self.lookup.container.remove(0).1)
        }
    }
}

#[derive(Clone)]
pub struct Iter<'a, V>
where
    V: LookupKey,
{
    lookup: &'a Lookup<V>,
    index: usize,
}

impl<'a, V> Iterator for Iter<'a, V>
where
    V: LookupKey,
{
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;
        self.lookup.container.get(index).map(|(_, v)| v)
    }
}

impl<'a, V> IntoIterator for &'a Lookup<V>
where
    V: LookupKey,
{
    type Item = &'a V;
    type IntoIter = Iter<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            lookup: self,
            index: 0,
        }
    }
}

/*
struct IterMut<'a, V>
where
    V: LookupKey,
{
    lookup: &'a mut Lookup<V>,
    index: usize,
}

impl<'a, V> Iterator for IterMut<'a, V>
where
    V: LookupKey,
{
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;
        let mut o = self.lookup
            .container
            .get_mut(index)
            .map(move |(_, v)| v);
        &mut o
    }
}

impl<'a, V> IntoIterator for &'a mut Lookup<V>
where
    V: LookupKey,
{
    type Item = &'a mut V;
    type IntoIter = IterMut<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            lookup: self,
            index: 0,
        }
    }
}

 */

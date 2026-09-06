// SPDX-FileCopyrightText: 2026 rstared contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::convert::AsRef;

use maplike::abc::{Container, Keyed};
use maplike::iter::IntoIter;
use maplike::ops::{Get, Insert, Push, Remove, Set};
use rstar::{RTree, RTreeObject, primitives::GeomWithData};

#[derive(Clone, Debug, Default)]
pub struct RTreed<C: Keyed>
where
    C::Value: RTreeObject,
{
    collection: C,
    rtree: RTree<GeomWithData<C::Value, C::Key>>,
}

impl<C: Keyed> AsRef<RTree<GeomWithData<C::Value, C::Key>>> for RTreed<C>
where
    C::Value: RTreeObject,
{
    fn as_ref(&self) -> &RTree<GeomWithData<C::Value, C::Key>> {
        &self.rtree
    }
}

impl<C: Keyed> RTreed<C>
where
    C::Value: RTreeObject,
{
    #[inline]
    pub fn new(collection: C) -> Self {
        Self {
            collection,
            rtree: RTree::new(),
        }
    }

    #[inline]
    pub fn collection(&self) -> &C {
        &self.collection
    }

    #[inline]
    pub fn rtree(&self) -> &RTree<GeomWithData<C::Value, C::Key>> {
        &self.rtree
    }
}

impl<C: Keyed> Container for RTreed<C>
where
    C::Value: RTreeObject,
{
    type Value = C::Value;
}

impl<C: Keyed> Keyed for RTreed<C>
where
    C::Value: RTreeObject,
{
    type Key = C::Key;
}

impl<K, C> Get<K> for RTreed<C>
where
    C: Keyed<Key = K> + Get<K>,
    C::Value: RTreeObject,
{
    #[inline]
    fn get(&self, key: &K) -> Option<&C::Value> {
        self.get(key)
    }
}

impl<K, C> RTreed<C>
where
    C: Keyed<Key = K> + Get<K>,
    C::Value: RTreeObject,
{
    #[inline]
    pub fn get(&self, key: &K) -> Option<&C::Value> {
        self.collection.get(key)
    }
}

impl<K, C> Insert<K> for RTreed<C>
where
    K: Clone,
    C: Keyed<Key = K> + Get<K> + Insert<K>,
    C::Value: Clone + RTreeObject,
{
    type Output = <C as Insert<K>>::Output;

    #[inline]
    fn insert(&mut self, key: K, value: C::Value) -> Self::Output {
        self.insert(key, value)
    }
}

impl<K, C> RTreed<C>
where
    K: Clone,
    C: Keyed<Key = K> + Get<K> + Insert<K>,
    C::Value: Clone + RTreeObject,
{
    #[inline]
    pub fn insert(&mut self, key: K, value: C::Value) -> <C as Insert<K>>::Output {
        self.rtree
            .insert(GeomWithData::new(value.clone(), key.clone()));
        self.collection.insert(key, value)
    }
}

impl<K, C> Set<K> for RTreed<C>
where
    K: Clone + PartialEq,
    C: Keyed<Key = K> + Set<K> + Insert<K>,
    C::Value: Clone + PartialEq + RTreeObject,
{
    type Output = <C as Set<K>>::Output;

    #[inline]
    fn set(&mut self, key: K, value: C::Value) -> Self::Output {
        RTreed::set(self, key, value)
    }
}

impl<K, C> RTreed<C>
where
    K: Clone + PartialEq,
    C: Keyed<Key = K> + Set<K>,
    C::Value: Clone + PartialEq + RTreeObject,
{
    #[inline]
    pub fn set(&mut self, key: K, value: C::Value) -> <C as Set<K>>::Output {
        self.rtree
            .remove(&GeomWithData::new(value.clone(), key.clone()));
        self.rtree
            .insert(GeomWithData::new(value.clone(), key.clone()));
        self.collection.set(key, value)
    }
}

impl<K, V, C> Remove<K> for RTreed<C>
where
    K: Clone + PartialEq,
    V: Clone + PartialEq + RTreeObject,
    C: Keyed<Key = K, Value = V> + Remove<K, Output = Option<V>>,
{
    type Output = Option<V>;

    #[inline]
    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove(key)
    }
}

impl<K, V, C> RTreed<C>
where
    K: Clone + PartialEq,
    V: Clone + PartialEq + RTreeObject,
    C: Keyed<Key = K, Value = V> + Remove<K, Output = Option<V>>,
{
    #[inline]
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let value = self.collection.remove(&key.clone())?;
        self.rtree
            .remove(&GeomWithData::new(value.clone(), key.clone()));

        Some(value)
    }
}

impl<K, C> Push<K> for RTreed<C>
where
    K: Clone,
    C: Keyed<Key = K> + Push<K>,
    C::Value: Clone + RTreeObject,
{
    #[inline]
    fn push(&mut self, value: C::Value) -> K {
        self.push(value)
    }
}

impl<K, C> RTreed<C>
where
    K: Clone,
    C: Keyed<Key = K> + Push<K>,
    C::Value: Clone + RTreeObject,
{
    #[inline]
    pub fn push(&mut self, value: C::Value) -> K {
        let key = self.collection.push(value.clone());
        self.rtree.insert(GeomWithData::new(value, key.clone()));

        key
    }
}

impl<K, C> IntoIter<K> for RTreed<C>
where
    C: Keyed<Key = K> + IntoIter<K>,
    C::Value: RTreeObject,
{
    type IntoIter = C::IntoIter;

    #[inline]
    fn into_iter(self) -> C::IntoIter {
        self.collection.into_iter()
    }
}

// SPDX-FileCopyrightText: 2025 rstared contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

mod maplike;

use rstar::{RTree, RTreeObject, primitives::GeomWithData};

use crate::maplike::{Get, Insert, Keyed, Map, Push, Remove};

pub struct RTreed<K, V: RTreeObject, C> {
    collection: C,
    rtree: RTree<GeomWithData<V, K>>,
}

impl<K, V, C> Map for RTreed<K, V, C> {
    type Item = V;
}

impl<K, V, C> Keyed for RTreed<K, V, C> {
    type Key = K;
}

impl<K, V, C: Get<K, Item = V>> Get<K> for RTreed<K, V, C> {
    #[inline(always)]
    fn get(&self, key: &K) -> Option<&V> {
        self.collection.get(key)
    }
}

impl<K, V: RTreeObject, C: Get<K, Item = V> + Insert<K>> Insert<K> for RTreed<K, V, C> {
    #[inline(always)]
    fn insert(&mut self, key: K, value: V) {
        self.rtree.insert(GeomWithData::new(value, key));
        self.collection.insert(key, value);
    }
}

impl<K: PartialEq, V: PartialEq + RTreeObject, C: Remove<K, Item = V>> Remove<K>
    for RTreed<K, V, C>
{
    #[inline(always)]
    fn remove(&mut self, key: &K) -> Option<V> {
        let value = self.collection.remove(key)?;
        self.rtree.remove(&GeomWithData::new(value, *key));

        Some(value)
    }
}

impl<K: Clone, V: Clone + RTreeObject, C: Push<K, Item = V>> Push<K> for RTreed<K, V, C> {
    #[inline(always)]
    fn push(&mut self, value: V) -> K {
        let key = self.collection.push(value.clone());
        self.rtree.insert(GeomWithData::new(value, key.clone()));

        key
    }
}

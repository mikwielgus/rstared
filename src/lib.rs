// SPDX-FileCopyrightText: 2025 rstared contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![doc(html_root_url = "https://docs.rs/rstared")]
#![doc = include_str!("../README.md")]
//#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

// No feature for `alloc` because it would be always enabled anyway.
extern crate alloc;

use maplike::{IntoIter, Set};
use rstar::{RTree, RTreeObject, primitives::GeomWithData};

pub use maplike::{Get, Insert, KeyedCollection, Push, Remove, StableRemove};

#[derive(Clone, Debug)]
pub struct RTreed<C: KeyedCollection>
where
    C::Value: RTreeObject,
{
    collection: C,
    rtree: RTree<GeomWithData<C::Value, C::Key>>,
}

impl<C: KeyedCollection> RTreed<C>
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

impl<C: KeyedCollection + Default> Default for RTreed<C>
where
    C::Value: RTreeObject,
{
    #[inline]
    fn default() -> Self {
        RTreed::new(C::default())
    }
}

impl<C: KeyedCollection> KeyedCollection for RTreed<C>
where
    C::Value: RTreeObject,
{
    type Value = C::Value;
    type Key = C::Key;
}

impl<K, C: Get<K, Key = K>> Get<K> for RTreed<C>
where
    C::Value: RTreeObject,
{
    #[inline]
    fn get(&self, key: &K) -> Option<&C::Value> {
        self.get(key)
    }
}

impl<K, C: Get<K, Key = K>> RTreed<C>
where
    C::Value: RTreeObject,
{
    #[inline]
    pub fn get(&self, key: &K) -> Option<&C::Value> {
        self.collection.get(key)
    }
}

impl<K, C: Get<K, Key = K> + Insert<K>> Insert<K> for RTreed<C>
where
    K: Clone,
    C::Value: Clone + RTreeObject,
{
    #[inline]
    fn insert(&mut self, key: K, value: C::Value) {
        self.insert(key, value);
    }
}

impl<K, C: Get<K, Key = K> + Insert<K>> RTreed<C>
where
    K: Clone,
    C::Value: Clone + RTreeObject,
{
    #[inline]
    pub fn insert(&mut self, key: K, value: C::Value) {
        self.rtree
            .insert(GeomWithData::new(value.clone(), key.clone()));
        self.collection.insert(key, value);
    }
}

impl<K, C: Set<K, Key = K> + Insert<K>> Set<K> for RTreed<C>
where
    K: Clone + PartialEq,
    C::Value: Clone + PartialEq + RTreeObject,
{
    #[inline]
    fn set(&mut self, key: K, value: C::Value) {
        RTreed::set(self, key, value);
    }
}

impl<K, C: Set<K, Key = K> + Set<K>> RTreed<C>
where
    K: Clone + PartialEq,
    C::Value: Clone + PartialEq + RTreeObject,
{
    #[inline]
    pub fn set(&mut self, key: K, value: C::Value) {
        self.rtree
            .remove(&GeomWithData::new(value.clone(), key.clone()));
        self.rtree
            .insert(GeomWithData::new(value.clone(), key.clone()));
        self.collection.set(key, value);
    }
}

impl<K, C: StableRemove<K, Key = K>> Remove<K> for RTreed<C>
where
    K: Clone + PartialEq,
    C::Value: Clone + PartialEq + RTreeObject,
{
    #[inline]
    fn remove(&mut self, key: &K) -> Option<C::Value> {
        self.remove(key)
    }
}

impl<K, C: StableRemove<K, Key = K>> StableRemove<K> for RTreed<C>
where
    K: Clone + PartialEq,
    C::Value: Clone + PartialEq + RTreeObject,
{
}

impl<K, C: StableRemove<K, Key = K>> RTreed<C>
where
    K: Clone + PartialEq,
    C::Value: Clone + PartialEq + RTreeObject,
{
    #[inline]
    pub fn remove(&mut self, key: &K) -> Option<C::Value> {
        let value = self.collection.remove(&key.clone())?;
        self.rtree
            .remove(&GeomWithData::new(value.clone(), key.clone()));

        Some(value)
    }
}

impl<K, C: Push<K, Key = K>> Push<K> for RTreed<C>
where
    K: Clone,
    C::Value: Clone + RTreeObject,
{
    #[inline]
    fn push(&mut self, value: C::Value) -> K {
        self.push(value)
    }
}

impl<K, C: Push<K, Key = K>> RTreed<C>
where
    K: Clone,
    C::Value: Clone + RTreeObject,
{
    #[inline]
    pub fn push(&mut self, value: C::Value) -> K {
        let key = self.collection.push(value.clone());
        self.rtree.insert(GeomWithData::new(value, key.clone()));

        key
    }
}

impl<K, C: IntoIter<K, Key = K>> IntoIter<K> for RTreed<C>
where
    C::Value: RTreeObject,
{
    type IntoIter = C::IntoIter;

    #[inline]
    fn into_iter(self) -> C::IntoIter {
        self.collection.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use rstar::{AABB, primitives::Rectangle};

    use super::*;

    #[cfg(feature = "stable-vec")]
    #[test]
    fn test_push_and_remove_random_aars_in_stable_vec() {
        use stable_vec::StableVec;
        test_push_and_remove_random_aars::<usize, StableVec<Rectangle<(i32, i32)>>>(StableVec::<
            Rectangle<(i32, i32)>,
        >::new(
        ));
    }

    #[cfg(feature = "thunderdome")]
    #[test]
    fn test_push_and_remove_random_aars_in_thunderdome() {
        use thunderdome::Arena;
        test_push_and_remove_random_aars::<thunderdome::Index, Arena<Rectangle<(i32, i32)>>>(
            Arena::<Rectangle<(i32, i32)>>::new(),
        );
    }

    /// "AAR" stands for "axis-aligned rectangle".
    fn test_push_and_remove_random_aars<
        K: Clone + PartialEq,
        C: KeyedCollection<Key = K, Value = Rectangle<(i32, i32)>>
            + Get<K>
            + Push<K>
            + StableRemove<K>,
    >(
        collection: C,
    ) {
        let mut rtreed: RTreed<C> = RTreed::new(collection);
        let mut rng = rand::rng();
        let mut keys = alloc::vec![];

        for _ in 0..100 {
            let x = rng.random_range(0..=90);
            let y = rng.random_range(0..=90);
            let width = rng.random_range(0..=10);
            let height = rng.random_range(0..=10);

            keys.push(rtreed.push(Rectangle::from_corners((x, y), (x + width, y + height))));
        }

        assert_eq!(
            rtreed
                .rtree()
                .locate_in_envelope(&AABB::from_corners((0, 0), (100, 100)))
                .count(),
            100
        );

        for i in 0..10 {
            rtreed.remove(&keys[i]);
        }

        assert_eq!(
            rtreed
                .rtree()
                .locate_in_envelope(&AABB::from_corners((0, 0), (100, 100)))
                .count(),
            90
        );
    }
}

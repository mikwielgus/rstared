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

mod maplike;

use rstar::{RTree, RTreeObject, primitives::GeomWithData};

use crate::maplike::{Get, Insert, Keyed, Map, Push, Remove};

pub struct RTreed<K, V: RTreeObject, C> {
    collection: C,
    rtree: RTree<GeomWithData<V, K>>,
}

impl<K, V: RTreeObject, C> RTreed<K, V, C> {
    pub fn new(collection: C) -> Self {
        Self {
            collection,
            rtree: RTree::new(),
        }
    }
}

impl<K, V: RTreeObject, C> RTreed<K, V, C> {
    pub fn collection(&self) -> &C {
        &self.collection
    }

    pub fn rtree(&self) -> &RTree<GeomWithData<V, K>> {
        &self.rtree
    }
}

impl<K, V: RTreeObject, C> Map for RTreed<K, V, C> {
    type Item = V;
}

impl<K, V: RTreeObject, C> Keyed for RTreed<K, V, C> {
    type Key = K;
}

impl<K, V: RTreeObject, C: Get<K, Item = V>> Get<K> for RTreed<K, V, C> {
    #[inline(always)]
    fn get(&self, key: &K) -> Option<&V> {
        self.collection.get(key)
    }
}

impl<K: Clone, V: Clone + RTreeObject, C: Get<K, Item = V> + Insert<K>> Insert<K>
    for RTreed<K, V, C>
{
    #[inline(always)]
    fn insert(&mut self, key: K, value: V) {
        self.rtree
            .insert(GeomWithData::new(value.clone(), key.clone()));
        self.collection.insert(key, value);
    }
}

impl<K: Clone + PartialEq, V: Clone + PartialEq + RTreeObject, C: Remove<K, Item = V>> Remove<K>
    for RTreed<K, V, C>
{
    #[inline(always)]
    fn remove(&mut self, key: &K) -> Option<V> {
        let value = self.collection.remove(&key.clone())?;
        self.rtree
            .remove(&GeomWithData::new(value.clone(), key.clone()));

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

#[cfg(test)]
mod tests {
    use std::vec;

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
        use thunderdome::{Arena, Index};
        test_push_and_remove_random_aars::<Index, Arena<Rectangle<(i32, i32)>>>(Arena::<
            Rectangle<(i32, i32)>,
        >::new());
    }

    /// "AAR" stands for "axis-aligned rectangle".
    fn test_push_and_remove_random_aars<
        K: Clone + PartialEq,
        C: Get<K, Item = Rectangle<(i32, i32)>> + Push<K> + Remove<K>,
    >(
        collection: C,
    ) {
        let mut rtreed: RTreed<K, Rectangle<(i32, i32)>, C> = RTreed::new(collection);
        let mut rng = rand::rng();
        let mut keys = vec![];

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

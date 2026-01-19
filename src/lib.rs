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

use maplike::IntoIter;
use rstar::{RTree, RTreeObject, primitives::GeomWithData};

pub use maplike::{Get, Insert, Keyed, Map, Push, Remove, StableRemove};

#[derive(Clone, Debug)]
pub struct RTreed<C: Keyed + Map>
where
    C::Item: RTreeObject,
{
    collection: C,
    rtree: RTree<GeomWithData<C::Item, C::Key>>,
}

impl<C: Keyed + Map> RTreed<C>
where
    C::Item: RTreeObject,
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
    pub fn rtree(&self) -> &RTree<GeomWithData<C::Item, C::Key>> {
        &self.rtree
    }
}

impl<C: Keyed + Map + Default> Default for RTreed<C>
where
    C::Item: RTreeObject,
{
    #[inline]
    fn default() -> Self {
        RTreed::new(C::default())
    }
}

impl<C: Keyed + Map> Map for RTreed<C>
where
    C::Item: RTreeObject,
{
    type Item = C::Item;
}

impl<C: Keyed + Map> Keyed for RTreed<C>
where
    C::Item: RTreeObject,
{
    type Key = C::Key;
}

impl<C: Keyed + Map + Get<C::Key>> Get<C::Key> for RTreed<C>
where
    C::Item: RTreeObject,
{
    #[inline]
    fn get(&self, key: &C::Key) -> Option<&C::Item> {
        self.get(key)
    }
}

impl<C: Keyed + Map + Get<C::Key>> RTreed<C>
where
    C::Item: RTreeObject,
{
    #[inline]
    pub fn get(&self, key: &C::Key) -> Option<&C::Item> {
        self.collection.get(key)
    }
}

impl<C: Keyed + Map + Get<C::Key> + Insert<C::Key>> Insert<C::Key> for RTreed<C>
where
    C::Key: Clone,
    C::Item: Clone + RTreeObject,
{
    #[inline]
    fn insert(&mut self, key: C::Key, value: C::Item) {
        self.insert(key, value);
    }
}

impl<C: Keyed + Map + Get<C::Key> + Insert<C::Key>> RTreed<C>
where
    C::Key: Clone,
    C::Item: Clone + RTreeObject,
{
    #[inline]
    pub fn insert(&mut self, key: C::Key, value: C::Item) {
        self.rtree
            .insert(GeomWithData::new(value.clone(), key.clone()));
        self.collection.insert(key, value);
    }
}

impl<C: Keyed + Map + StableRemove<C::Key>> Remove<C::Key> for RTreed<C>
where
    C::Key: Clone + PartialEq,
    C::Item: Clone + PartialEq + RTreeObject,
{
    #[inline]
    fn remove(&mut self, key: &C::Key) -> Option<C::Item> {
        self.remove(key)
    }
}

impl<C: Keyed + Map + StableRemove<C::Key>> StableRemove<C::Key> for RTreed<C>
where
    C::Key: Clone + PartialEq,
    C::Item: Clone + PartialEq + RTreeObject,
{
}

impl<C: Keyed + Map + StableRemove<C::Key>> RTreed<C>
where
    C::Key: Clone + PartialEq,
    C::Item: Clone + PartialEq + RTreeObject,
{
    #[inline]
    pub fn remove(&mut self, key: &C::Key) -> Option<C::Item> {
        let value = self.collection.remove(&key.clone())?;
        self.rtree
            .remove(&GeomWithData::new(value.clone(), key.clone()));

        Some(value)
    }
}

impl<C: Keyed + Map + Push<C::Key>> Push<C::Key> for RTreed<C>
where
    C::Key: Clone,
    C::Item: Clone + RTreeObject,
{
    #[inline]
    fn push(&mut self, value: C::Item) -> C::Key {
        self.push(value)
    }
}

impl<C: Keyed + Map + Push<C::Key>> RTreed<C>
where
    C::Key: Clone,
    C::Item: Clone + RTreeObject,
{
    #[inline]
    pub fn push(&mut self, value: C::Item) -> C::Key {
        let key = self.collection.push(value.clone());
        self.rtree.insert(GeomWithData::new(value, key.clone()));

        key
    }
}

impl<C: Keyed + Map + IntoIter<<C as Keyed>::Key>> IntoIter<<C as Keyed>::Key> for RTreed<C>
where
    C::Item: RTreeObject,
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
        test_push_and_remove_random_aars::<StableVec<Rectangle<(i32, i32)>>>(StableVec::<
            Rectangle<(i32, i32)>,
        >::new());
    }

    #[cfg(feature = "thunderdome")]
    #[test]
    fn test_push_and_remove_random_aars_in_thunderdome() {
        use thunderdome::Arena;
        test_push_and_remove_random_aars::<Arena<Rectangle<(i32, i32)>>>(Arena::<
            Rectangle<(i32, i32)>,
        >::new());
    }

    /// "AAR" stands for "axis-aligned rectangle".
    fn test_push_and_remove_random_aars<
        C: Keyed
            + Map<Item = Rectangle<(i32, i32)>>
            + Get<C::Key>
            + Push<C::Key>
            + StableRemove<C::Key>,
    >(
        collection: C,
    ) where
        C::Key: Clone + PartialEq,
    {
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

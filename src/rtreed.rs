// SPDX-FileCopyrightText: 2026 rstared contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::convert::AsRef;

use maplike::containers::Container;
use maplike::iter::IntoIter;
use maplike::ops::{Get, Insert, Push, Remove, Set};
use rstar::{RTree, RTreeObject, primitives::GeomWithData};

#[derive(Clone, Debug, Default)]
pub struct RTreed<C: Container>
where
    C::Value: RTreeObject,
{
    collection: C,
    rtree: RTree<GeomWithData<C::Value, C::Key>>,
}

impl<C: Container> AsRef<RTree<GeomWithData<C::Value, C::Key>>> for RTreed<C>
where
    C::Value: RTreeObject,
{
    fn as_ref(&self) -> &RTree<GeomWithData<C::Value, C::Key>> {
        &self.rtree
    }
}

impl<C: Container> RTreed<C>
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

impl<C: Container> Container for RTreed<C>
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
    type Output = <C as Insert<K>>::Output;

    #[inline]
    fn insert(&mut self, key: K, value: C::Value) -> Self::Output {
        self.insert(key, value)
    }
}

impl<K, C: Get<K, Key = K> + Insert<K>> RTreed<C>
where
    K: Clone,
    C::Value: Clone + RTreeObject,
{
    #[inline]
    pub fn insert(&mut self, key: K, value: C::Value) -> C::Output {
        self.rtree
            .insert(GeomWithData::new(value.clone(), key.clone()));
        self.collection.insert(key, value)
    }
}

impl<K, C: Set<K, Key = K> + Insert<K>> Set<K> for RTreed<C>
where
    K: Clone + PartialEq,
    C::Value: Clone + PartialEq + RTreeObject,
{
    type Output = <C as Set<K>>::Output;

    #[inline]
    fn set(&mut self, key: K, value: C::Value) -> Self::Output {
        RTreed::set(self, key, value)
    }
}

impl<K, C: Set<K, Key = K> + Set<K>> RTreed<C>
where
    K: Clone + PartialEq,
    C::Value: Clone + PartialEq + RTreeObject,
{
    #[inline]
    pub fn set(&mut self, key: K, value: C::Value) -> C::Output {
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
    C: Container<Key = K, Value = V> + Remove<K, Output = Option<V>>,
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
    C: Container<Key = K, Value = V> + Remove<K, Output = Option<V>>,
{
    #[inline]
    pub fn remove(&mut self, key: &K) -> Option<V> {
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

#[cfg(all(test, any(feature = "stable-vec", feature = "thunderdome")))]
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
        C: Container<Key = K, Value = Rectangle<(i32, i32)>>
            + Get<K>
            + Push<K>
            + Remove<K, Output = Option<Rectangle<(i32, i32)>>>,
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
                .locate_in_envelope(AABB::from_corners((0, 0), (100, 100)))
                .count(),
            100
        );

        for i in 0..10 {
            rtreed.remove(&keys[i]);
        }

        assert_eq!(
            rtreed
                .rtree()
                .locate_in_envelope(AABB::from_corners((0, 0), (100, 100)))
                .count(),
            90
        );
    }
}

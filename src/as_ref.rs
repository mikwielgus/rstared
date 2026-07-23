// SPDX-FileCopyrightText: 2026 rstared contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::convert::AsRef;

use maplike::containers::Container;
use maplike::iter::IntoIter;
use maplike::ops::{Get, Insert, Remove, Set};
use rstar::{RTree, RTreeObject};

#[derive(Clone, Debug)]
pub struct AsRefRTree<T: RTreeObject> {
    pub rtree: RTree<T>,
}

impl<T: RTreeObject> AsRef<RTree<T>> for AsRefRTree<T> {
    fn as_ref(&self) -> &RTree<T> {
        &self.rtree
    }
}

impl<T: RTreeObject> AsRefRTree<T> {
    pub fn new() -> Self {
        Self {
            rtree: RTree::new(),
        }
    }
}

impl<T: RTreeObject> Default for AsRefRTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: RTreeObject> Container for AsRefRTree<K> {
    type Key = K;
    type Value = ();
}

impl<K: RTreeObject + PartialEq> Get<K> for AsRefRTree<K> {
    #[inline(always)]
    fn get(&self, key: &K) -> Option<&()> {
        Get::get(&self.rtree, key)
    }
}

impl<K: RTreeObject + PartialEq> Set<K> for AsRefRTree<K> {
    type Output = ();

    #[inline(always)]
    fn set(&mut self, key: K, value: ()) -> () {
        Set::set(&mut self.rtree, key, value);
    }
}

impl<K: RTreeObject> Insert<K> for AsRefRTree<K> {
    type Output = ();

    #[inline(always)]
    fn insert(&mut self, key: K, value: ()) -> () {
        Insert::insert(&mut self.rtree, key, value)
    }
}

impl<K: RTreeObject + PartialEq> Remove<K> for AsRefRTree<K> {
    type Output = <RTree<K> as Remove<K>>::Output;

    #[inline(always)]
    fn remove(&mut self, key: &K) -> Self::Output {
        Remove::remove(&mut self.rtree, key)
    }
}

impl<K: RTreeObject> IntoIter<K> for AsRefRTree<K> {
    type IntoIter = <RTree<K> as IntoIter<K>>::IntoIter;

    #[inline(always)]
    fn into_iter(self) -> <RTree<K> as IntoIter<K>>::IntoIter {
        IntoIter::into_iter(self.rtree)
    }
}

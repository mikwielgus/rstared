// SPDX-FileCopyrightText: 2026 rstared contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc::collections::BTreeMap;

use maplike::containers::Container;
use maplike::iter::IntoIter;
use maplike::ops::{Get, Insert, Remove};
use rstar::RTreeObject;

use crate::RTreed;
use undoredo::{ApplyDelta, Delta};

/// Half-delta for `RTreed<C>`. Alias for `BTreeMap<K, V>`.
pub type RTreedHalfDelta<K, V> = BTreeMap<K, V>;

/// Delta for `RTreed<C>`. Alias for `Delta<RTreedHalfDelta<K, V>>`.
pub type RTreedDelta<K, V> = Delta<RTreedHalfDelta<K, V>>;

impl<
    K: Clone + PartialEq,
    V: Clone + PartialEq + RTreeObject,
    C: Get<K> + Container<Key = K, Value = V> + Insert<K> + Remove<K, Output = Option<V>>,
    DC: IntoIter<K> + Container<Key = K, Value = V>,
> ApplyDelta<DC> for RTreed<C>
{
    #[inline]
    fn apply_delta(&mut self, delta: Delta<DC>) {
        let (removed, inserted) = delta.dissolve();

        for (removed_key, _removed_value) in removed.into_iter() {
            self.remove(&removed_key);
        }

        for (inserted_key, inserted_value) in inserted.into_iter() {
            self.insert(inserted_key, inserted_value);
        }
    }
}

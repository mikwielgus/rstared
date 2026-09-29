// SPDX-FileCopyrightText: 2026 rstared contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc::collections::BTreeMap;

use maplike::abc::Keyed;
use maplike::iter::IntoIter;
use maplike::ops::{Insert, Remove};
use undoredo::{ApplyDelta, Delta};

use super::{RTree, RTreeObject};

/// Half-delta for `RTree<K>`. Alias for `BTreeMap<K, ()>`.
pub type RTreeHalfDelta<K> = BTreeMap<K, ()>;

/// Delta for `RTree<K>`. Alias for `Delta<RTreeHalfDelta<K>>`.
pub type RTreeDelta<K> = Delta<RTreeHalfDelta<K>>;

impl<K: RTreeObject + PartialEq, DC: IntoIter<K> + Keyed<Value = (), Key = K>> ApplyDelta<DC>
    for RTree<K>
{
    #[inline]
    fn apply_delta(&mut self, delta: Delta<DC>) {
        let (removed, inserted) = delta.dissolve();

        for (removed_key, _removed_value) in removed.into_iter() {
            Remove::remove(self, &removed_key);
        }

        for (inserted_key, inserted_value) in inserted.into_iter() {
            Insert::insert(self, inserted_key, inserted_value);
        }
    }
}

// SPDX-FileCopyrightText: 2026 rstared contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

extern crate alloc;

use maplike::{
    containers::Container,
    ops::{Get, Push, Remove},
};
use rand::Rng;
use rstar::{AABB, primitives::Rectangle};
use rstared::RTreed;

#[test]
fn test_push_and_remove_random_aars_in_stable_vec() {
    use stable_vec::StableVec;
    test_push_and_remove_random_aars::<usize, StableVec<Rectangle<(i32, i32)>>>(StableVec::<
        Rectangle<(i32, i32)>,
    >::new());
}

#[test]
fn test_push_and_remove_random_aars_in_thunderdome() {
    use thunderdome::Arena;
    test_push_and_remove_random_aars::<thunderdome::Index, Arena<Rectangle<(i32, i32)>>>(Arena::<
        Rectangle<(i32, i32)>,
    >::new(
    ));
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

// SPDX-FileCopyrightText: 2025 rstared contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::HashMap;

use rstar::{AABB, primitives::Rectangle};
use rstared::{Insert, RTreed, Remove};

fn main() {
    // A hashmap of 2D rectangles will be the underlying collection.
    let rect_hashmap: HashMap<i32, Rectangle<(i32, i32)>> = HashMap::new();

    // Wrap `RTreed` around the hashmap.
    let mut rtreed = RTreed::<i32, Rectangle<(i32, i32)>, HashMap<i32, Rectangle<(i32, i32)>>>::new(
        rect_hashmap,
    );

    // Insert two rectangles, recording them in the R-tree.
    rtreed.insert(1, Rectangle::from_corners((0, 0), (1, 1)));
    rtreed.insert(2, Rectangle::from_corners((1, 1), (2, 2)));

    // Locate the two rectangles in the R-tree.
    assert_eq!(
        rtreed
            .rtree()
            .locate_in_envelope(&AABB::from_corners((0, 0), (2, 2)))
            .count(),
        2
    );

    // Now remove one of the rectangles, recording this in the R-tree.
    rtreed.remove(&1);

    // Make the same query to the R-tree as before.
    // Only one rectangle is now present.
    assert_eq!(
        rtreed
            .rtree()
            .locate_in_envelope(&AABB::from_corners((0, 0), (2, 2)))
            .count(),
        1
    );
}

#[test]
fn test() {
    main();
}

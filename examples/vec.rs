// SPDX-FileCopyrightText: 2025 rstared contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use rstar::{AABB, primitives::Rectangle};
use rstared::RTreed;

fn main() {
    // A vec of 2D rectangles will be the underlying collection.
    let rect_vec: Vec<Rectangle<(i32, i32)>> = Vec::new();

    // Wrap `RTreed` around the vec.
    let mut rtreed = RTreed::new(rect_vec);

    // Push two rectangles, recording them in the R-tree.
    rtreed.push(Rectangle::from_corners((0, 0), (1, 1)));
    rtreed.push(Rectangle::from_corners((1, 1), (2, 2)));

    // Locate the two rectangles in the R-tree.
    assert_eq!(
        rtreed
            .rtree()
            .locate_in_envelope(AABB::from_corners((0, 0), (2, 2)))
            .count(),
        2
    );

    // Access a rectangle by its index in the vec.
    assert_eq!(
        rtreed.get(&0),
        Some(&Rectangle::from_corners((0, 0), (1, 1)))
    );
}

#[test]
fn test() {
    main();
}

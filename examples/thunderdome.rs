// SPDX-FileCopyrightText: 2026 rstared contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use geo_types::{Point, Polygon, line_string};
use rstar::AABB;
use rstared::RTreed;
use thunderdome::Arena;

fn main() {
    // A generational arena of 2D polygons from `geo` will be the underlying
    // collection.
    let polygon_arena: Arena<Polygon<f64>> = Arena::new();

    // Wrap `RTreed` around the arena.
    let mut rtreed = RTreed::new(polygon_arena);

    // Insert two polygons (two non-regular pentagons), recording them in the
    // R-tree.
    let first = rtreed.push(Polygon::new(
        line_string![
            (x: 0.0, y: 0.0),
            (x: 2.0, y: 0.5),
            (x: 2.5, y: 2.0),
            (x: 1.0, y: 2.5),
            (x: 0.0, y: 1.0),
            (x: 0.0, y: 0.0),
        ],
        vec![],
    ));
    rtreed.push(Polygon::new(
        line_string![
            (x: 2.0, y: 2.0),
            (x: 5.0, y: 2.5),
            (x: 4.5, y: 4.0),
            (x: 3.0, y: 5.0),
            (x: 2.0, y: 2.0),
        ],
        vec![],
    ));

    // Locate the two polygons in the R-tree.
    assert_eq!(
        rtreed
            .rtree()
            .locate_in_envelope(AABB::from_corners(
                Point::new(0.0, 0.0),
                Point::new(5.0, 5.0),
            ))
            .count(),
        2
    );

    // Now remove one of the polygons, recording this in the R-tree.
    rtreed.remove(&first);

    // Make the same query to the R-tree as before.
    // Only one polygon is now present.
    assert_eq!(
        rtreed
            .rtree()
            .locate_in_envelope(AABB::from_corners(
                Point::new(0.0, 0.0),
                Point::new(5.0, 5.0),
            ))
            .count(),
        1
    );
}

#[test]
fn test() {
    main();
}

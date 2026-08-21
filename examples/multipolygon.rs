// SPDX-FileCopyrightText: 2025 rstared contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use geo_types::{MultiPolygon, Point, Polygon, line_string};
use rstar::AABB;
use rstared::RTreed;

fn main() {
    let multipolygon: MultiPolygon<f64> = MultiPolygon::new(vec![]);
    let mut rtreed = RTreed::new(multipolygon);

    // Push two polygons, recording them in the R-tree.
    rtreed.push(Polygon::new(
        line_string![
            (x: 0.0, y: 0.0),
            (x: 1.0, y: 0.0),
            (x: 1.0, y: 1.0),
            (x: 0.0, y: 0.0),
        ],
        vec![],
    ));
    rtreed.push(Polygon::new(
        line_string![
            (x: 1.0, y: 1.0),
            (x: 2.0, y: 1.0),
            (x: 2.0, y: 2.0),
            (x: 1.0, y: 1.0),
        ],
        vec![],
    ));

    // Locate the two polygons in the R-tree.
    assert_eq!(
        rtreed
            .rtree()
            .locate_in_envelope(AABB::from_corners(
                Point::new(0.0, 0.0),
                Point::new(2.0, 2.0),
            ))
            .count(),
        2
    );

    // Access a polygon by its index in the MultiPolygon.
    assert_eq!(
        rtreed.get(&0),
        Some(&Polygon::new(
            line_string![
                (x: 0.0, y: 0.0),
                (x: 1.0, y: 0.0),
                (x: 1.0, y: 1.0),
                (x: 0.0, y: 0.0),
            ],
            vec![],
        ))
    );
}

#[test]
fn test() {
    main();
}

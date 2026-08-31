<!--
SPDX-FileCopyrightText: 2025 rstared contributors

SPDX-License-Identifier: MIT OR Apache-2.0
-->

[![Repository](https://img.shields.io/badge/repository-GitHub-0FBF3E)](https://github.com/mikwielgus/rstared)
[![Docs](https://docs.rs/rstared/badge.svg)](https://docs.rs/rstared/)
[![Crates.io](https://img.shields.io/crates/v/rstared.svg)](https://crates.io/crates/rstared)
[![MIT OR Apache 2.0](https://img.shields.io/crates/l/rstared.svg)](#licence)

# rstared

`rstared::RTreed` is a simple Rust
[decorator](https://en.wikipedia.org/wiki/Decorator_pattern) that adds a
passively listening R-tree,
[`rstar::RTree`](https://docs.rs/rstar/0.12.2/rstar/struct.RTree.html), to a
large number of common standard library and third-party collection types.

## Supported collections

### Standard library

- [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html),
  gated by the `std` feature (enabled by default);
- [`HashSet`](https://doc.rust-lang.org/stable/std/collections/struct.HashSet.html),
  gated by the `std` feature (enabled by default);
- [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html),
  not feature-gated;
- [`BTreeSet`](https://doc.rust-lang.org/stable/std/collections/struct.BTreeSet.html),
  not feature-gated;
- [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html), not feature-gated;
- [`VecDeque`](https://doc.rust-lang.org/alloc/collections/vec_deque/struct.VecDeque.html),
  not feature-gated.

### Third-party types

- [`bidimap::BiBTreeMap`](https://docs.rs/bidimap/latest/bidimap/), gated by the
  `bidimap` feature, and
  [`bidimap::BiHashMap`](https://docs.rs/bidimap/latest/bidimap/), which is
  additionally gated by the `std` feature;
- [`indexmap::IndexMap`](https://docs.rs/indexmap/latest/indexmap/map/struct.IndexMap.html)
  and [`indexmap::IndexSet`](https://docs.rs/indexmap/latest/indexmap/set/struct.IndexSet.html),
  gated by the `indexmap` feature;
- [`rstar::RTree`](https://docs.rs/rstar/0.12.2/rstar/index.html), not
  feature-gated;
- [`stable_vec::StableVec`](https://docs.rs/stable-vec/latest/stable_vec/),
  gated by the `stable-vec` feature;
- [`thunderdome::Arena`](https://docs.rs/thunderdome/latest/thunderdome/),
  gated by the `thunderdome` feature;
- [`arrayvec::ArrayVec`](https://docs.rs/arrayvec/latest/arrayvec/struct.ArrayVec.html),
  gated by the `arrayvec` feature;
- `smallvec::SmallVec`, gated by the `smallvec` feature;
- [`tinyvec::ArrayVec`](https://docs.rs/tinyvec/latest/tinyvec/struct.ArrayVec.html)
  and [`tinyvec::TinyVec`](https://docs.rs/tinyvec/latest/tinyvec/enum.TinyVec.html),
  gated by the `tinyvec` feature;
- geometry types from [`geo`](https://docs.rs/geo)/[`geo-types`](https://docs.rs/geo-types):
  [`LineString`](https://docs.rs/geo/latest/geo/struct.LineString.html),
  [`MultiPoint`](https://docs.rs/geo/latest/geo/struct.MultiPoint.html),
  [`MultiLineString`](https://docs.rs/geo/latest/geo/struct.MultiLineString.html),
  [`MultiPolygon`](https://docs.rs/geo/latest/geo/struct.MultiPolygon.html), and
  [`GeometryCollection`](https://docs.rs/geo/latest/geo/struct.GeometryCollection.html)
  gated by the `geo` feature;

This library is `no_std`-compatible and has no mandatory third-party
dependencies except for [`alloc`](https://doc.rust-lang.org/alloc/).

## Usage

### Adding dependency

Add `rstared` as a dependency to your `Cargo.toml` together with the features
that gate the collections you are going to use:

```toml
[dependencies]
rstared = { version = "0.14.1", features = [
    "arrayvec",
    "bidimap",
    "geo",
    "indexmap",
    "smallvec",
    "stable-vec",
    "thunderdome",
    "tinyvec",
] }
```

For the sake of demonstration, all feature flags are enabled in that snippet.
Remove those you don't need.

### Usage examples

#### `Vec` example

Following is a basic usage example on `Vec`
([examples/vec.rs](https://github.com/mikwielgus/rstared/blob/develop/examples/vec.rs)).
`Vec` is pushable, so values are added with `.push()` and keyed by their index:

```rust
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
```

#### `HashMap` example

Because `Vec` invalidates indices upon removal, there is no `.remove()` method
available for `RTreed<Vec<...>`. If you want to dynamically remove elements, you
can use a type with stable keys, such as Rust standard library's `HashMap` and
`BTreeMap`, like this:

```rust-ignore
let rect_vec: HashMap<Rectangle<(i32, i32)>> = Vec::new();
let mut rtreed = RTreed::new(rect_vec);
```

See
[examples/hashmap.rs](https://github.com/mikwielgus/rstared/blob/develop/examples/multipolygon.rs)
for a full usage example on `HashMap`.

Of course, map types are not as fast as `Vec`s. If you want to retain most
of `Vec`s performance while still being able to stably remove elements,
consider using third-party collections such as `indexmap::IndexMap`,
`stable_vec::StableVec`, `thunderdome::Arena` -- `RTreed` can decorate them just
as well.

#### `MultiPolygon` example

Following is a usage example on `geo`'s
[`MultiPolygon`](https://docs.rs/geo/latest/geo/geometry/struct.MultiPolygon.html)
([examples/multipolygon.rs](https://github.com/mikwielgus/rstared/blob/develop/examples/multipolygon.rs)).
To wrap `RTreed` over `MultiPolygon`, you need to enable the `rstar_0_13`
feature on [`geo-types`](https://docs.rs/geo-types), so that its element type,
`Polygon`, implements `RTreeObject`:

```rust
# #[cfg(feature = "geo")]
# {

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

# }
```



## Contributing

We welcome issues and pull requests from anyone both to our
[repository](https://github.com/mikwielgus/rstared) on GitHub.

If you would like `rstared` to work with a new collection type, please make
a contribution to [`maplike`](https://docs.rs/maplike/latest/maplike/), which
provides and implements the traits `rstared` relies on.

## Licence

### Outbound licence

`rstared` is dual-licensed as under either of

- [MIT license](./LICENSES/MIT.txt),
- [Apache License, Version 2.0](./LICENSES/Apache-2.0.txt),

at your option.

### Inbound licence

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this work by you will be dual-licensed as described above,
without any additional terms or conditions.

<!--
SPDX-FileCopyrightText: 2025 rstared contributors

SPDX-License-Identifier: MIT OR Apache-2.0
-->

[![Docs](https://docs.rs/rstared/badge.svg)](https://docs.rs/rstared/)
[![Crates.io](https://img.shields.io/crates/v/rstared.svg)](https://crates.io/crates/rstared)
[![MIT OR Apache 2.0](https://img.shields.io/crates/l/rstared.svg)](#licence)

# rstared

`rstared::RTreed` is a simple Rust
[decorator](https://en.wikipedia.org/wiki/Decorator_pattern) that adds a
passively listening R-tree,
[`rstar::RTree`](https://docs.rs/rstar/0.12.2/rstar/struct.RTree.html), to
collections that implement [`maplike`](https://docs.rs/maplike/latest/maplike/)
traits (with values that implement
[`rstar::RTreeObject`](https://docs.rs/rstar/latest/rstar/trait.RTreeObject.html)).

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

This library is `no_std`-compatible and has no mandatory third-party
dependencies except for [`alloc`](https://doc.rust-lang.org/alloc/).

## Usage

### Adding dependency

Add `rstared` as a dependency to your `Cargo.toml` together with the features
that gate the collections you are going to use:

```toml
[dependencies]
rstared = { version = "0.14.0", features = [
    "arrayvec",
    "bidimap",
    "indexmap",
    "smallvec",
    "stable-vec",
    "thunderdome",
    "tinyvec",
] }
```

For the sake of demonstration, all feature flags are enabled in that snippet.
Remove those you don't need.

### Usage example on `HashMap`

Following is a basic usage example on `HashMap`
([examples/hashmap.rs](https://github.com/mikwielgus/rstared/blob/develop/examples/hashmap.rs)):

```rust
use std::collections::HashMap;

use rstar::{AABB, primitives::Rectangle};
use rstared::RTreed;

fn main() {
    // A hashmap of 2D rectangles will be the underlying collection.
    let rect_hashmap: HashMap<i32, Rectangle<(i32, i32)>> = HashMap::new();

    // Wrap `RTreed` around the hashmap.
    let mut rtreed = RTreed::new(rect_hashmap);

    // Insert two rectangles, recording them in the R-tree.
    rtreed.insert(1, Rectangle::from_corners((0, 0), (1, 1)));
    rtreed.insert(2, Rectangle::from_corners((1, 1), (2, 2)));

    // Locate the two rectangles in the R-tree.
    assert_eq!(
        rtreed
            .rtree()
            .locate_in_envelope(AABB::from_corners((0, 0), (2, 2)))
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
            .locate_in_envelope(AABB::from_corners((0, 0), (2, 2)))
            .count(),
        1
    );
}
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

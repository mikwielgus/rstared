<!--
SPDX-FileCopyrightText: 2025 rstared contributors

SPDX-License-Identifier: MIT OR Apache-2.0
-->

[![CI Status](https://ci.codeberg.org/api/badges/15785/status.svg)](https://ci.codeberg.org/repos/15785)
[![Docs](https://docs.rs/rstared/badge.svg)](https://docs.rs/rstared/)
[![Crates.io](https://img.shields.io/crates/v/rstared.svg)](https://crates.io/crates/rstared)
[![MIT OR Apache 2.0](https://img.shields.io/crates/l/rstared.svg)](#licence)

# rstared

`rstared::RTree` is a simple
[decorator](https://en.wikipedia.org/wiki/Decorator_pattern) that adds
[`rstar::RTree`](https://docs.rs/rstar/0.12.2/rstar/struct.RTree.html) to the
following collections:

- [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html), gated by the `std` feature (enabled by default);
- [`HashSet`](https://doc.rust-lang.org/stable/std/collections/struct.HashSet.html), gated by the `std` feature (enabled by default);
- [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html), not feature-gated;
- [`BTreeSet`](https://doc.rust-lang.org/stable/std/collections/struct.BTreeSet.html), not feature-gated;
- [`stable_vec::StableVec`](https://docs.rs/stable-vec/latest/stable_vec/),
  gated by the `stable-vec` feature; (example usage:
  [examples/stable_vec.rs](./examples/stable_vec.rs)),
- [`thunderdome::Arena`](https://docs.rs/thunderdome/latest/thunderdome/),
  gated by the `thunderdome` feature. (example usage:
  [examples/thunderdome.rs](./examples/thunderdome.rs)).

## Usage

### Basic usage

Add `rstared` as a dependency to your `Cargo.toml` together with the features
that gate the collections you are going to use. For example, to use `rstared`
with `stable_vec::StableVec` and `thunderdome::Arena`, write

```toml
[dependencies]
rstared = { version = "0.1", features = ["stable-vec", "thunderdome"] }
```

Following is a basic usage example taken from
[examples/basic_usage.rs](./examples/basic_usage.rs):

```rust
use std::collections::HashMap;

use rstar::{AABB, primitives::Rectangle};
use rstared::{Insert, RTreed, Remove};

fn main() {
    // A hashmap of 2D rectangles will be the underlying collection.
    let rect_hashmap: HashMap<i32, Rectangle<(i32, i32)>> = HashMap::new();

    // Wrap `RTreed` around the hashmap.
    let mut rtreed =
        RTreed::<i32, Rectangle<(i32, i32)>, HashMap<i32, Rectangle<(i32, i32)>>>::new(
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
```

### Usage on maps with pushing

Some data structures with map semantics also provide a special type of insertion
where a value is inserted without specifying a key, which the structure
instead automatically generates and returns by itself. This operation is called
"pushing".

If a supported type has a push interface, you can use it through `RTreed` by
calling [`.push()`], like this:

```rust-ignore
rtreed.push('A');
```

`StableVec` and `thunderdome::Arena` are instances of supported pushable maps.

## Contributing

We welcome issues and pull requests from anyone both to our canonical
[repository](https://codeberg.org/topola/rstared) on Codeberg and to our GitHub
[mirror](https://github.com/mikwielgus/rstared).

**NOTE:** This repository currently contains a Git submodule:
`src/maplike`, which we currently use to share functionality with the
[`undoredo`](https://docs.rs/undoredo/) crate to avoid creating another crate
for now. After `git clone`, remember to run

```bash
git submodule update --init
```

in the cloned directory.

## Licence

`undoredo` is dual-licensed as under either of

- [MIT license](./LICENSES/MIT.txt),
- [Apache License, Version 2.0](./LICENSES/Apache-2.0.txt),

at your option.

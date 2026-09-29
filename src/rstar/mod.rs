//! An n-dimensional [r*-tree](https://en.wikipedia.org/wiki/R*-tree) implementation for use as a spatial index.
//!
//! Vendored into `rstared` as [`crate::rstar`]. Originally from the
//! [`rstar`](https://crates.io/crates/rstar) crate (MIT OR Apache-2.0).
//!
//! # R-Tree
//! An r-tree is a data structure containing _spatial data_, optimized for
//! nearest neighbor search.
//! _Spatial data_ refers to an object that has the notion of a position and extent:
//! for example points, lines and rectangles in any dimension.
//!
//! # Further documentation
//! The module's main data structure and documentation is the [RTree] struct.
//!
//! ## Primitives
//! The pre-defined primitives like lines and rectangles contained in
//! the [primitives module](crate::rstar::primitives) may be of interest for a quick start.
//!
//! ## `Geo`
//! Enable the crate's `geo` feature so that
//! [`geo-types`](https://docs.rs/geo-types) geometries implement [`RTreeObject`].
//!
//! # (De)Serialization
//! Enable the `serde` feature for [serde](https://crates.io/crates/serde) support.
//!
//! # Mint compatibility with other crates
//! Enable the `mint` feature for
//! [`mint`](https://crates.io/crates/mint) support. See the
//! documentation on the [mint] module for an expample of an
//! integration with the
//! [`nalgebra`](https://crates.io/crates/nalgebra) crate.
#![deny(missing_docs)]
#![forbid(unsafe_code)]

mod aabb;
mod algorithm;
#[cfg(feature = "undoredo")]
mod delta;
mod envelope;
mod maplike;
mod node;
mod object;
mod params;
mod point;
pub mod primitives;
mod rtree;

#[cfg(feature = "mint")]
pub mod mint;

#[cfg(feature = "geo")]
mod geo_types;

#[cfg(test)]
mod test_utilities;

pub use crate::rstar::aabb::AABB;
pub use crate::rstar::algorithm::rstar::RStarInsertionStrategy;
pub use crate::rstar::algorithm::selection_functions::SelectionFunction;
pub use crate::rstar::envelope::Envelope;
pub use crate::rstar::node::{ParentNode, RTreeNode};
pub use crate::rstar::object::{PointDistance, RTreeObject};
pub use crate::rstar::params::{DefaultParams, InsertionStrategy, RTreeParams};
pub use crate::rstar::point::{Point, RTreeNum};
pub use crate::rstar::rtree::RTree;

#[cfg(feature = "undoredo")]
pub use crate::rstar::delta::{RTreeDelta, RTreeHalfDelta};

pub use crate::rstar::algorithm::iterators;

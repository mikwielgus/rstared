// SPDX-FileCopyrightText: 2025 rstared contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![doc(html_root_url = "https://docs.rs/rstared")]
#![doc = include_str!("../README.md")]
//#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

// No feature for `alloc` because it would be always enabled anyway.
extern crate alloc;

mod as_ref;
mod rtreed;

pub use crate::as_ref::AsRefRTree;
pub use crate::rtreed::RTreed;
pub use maplike::{Get, Insert, KeyedCollection, Push, Remove, StableRemove};

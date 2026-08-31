// SPDX-FileCopyrightText: 2025 rstared contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![doc(html_root_url = "https://docs.rs/rstared")]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
//#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![no_std]

// No feature for `alloc` because it would be always enabled anyway.
extern crate alloc;

#[cfg(feature = "undoredo")]
mod delta;
#[cfg(feature = "undoredo")]
pub use delta::*;

mod rtreed;

pub use crate::rtreed::RTreed;
pub use maplike::containers::Container;
pub use maplike::ops::{Get, Insert, Push, Remove};

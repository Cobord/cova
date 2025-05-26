#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![allow(
  clippy::must_use_candidate,
  clippy::return_self_not_must_use,
  clippy::doc_markdown,
  clippy::wildcard_imports
)]

pub mod cloud;
pub mod complexes;
pub mod definitions;
pub mod filtration;
pub mod graph;
pub mod homology;
pub mod lattice;
pub mod set;
pub mod sheaf;

use cova_algebra::prelude::*;

pub mod prelude {
  //! The prelude for the `space` crate.
  //!
  //! This module re-exports the most commonly used types and traits from the `space` crate.
  //! It provides a convenient way to import these types and traits into your code without
  //! having to specify the crate name each time.
  pub use crate::{
    complexes::ComplexElement,
    definitions::{MetricSpace, NormedSpace, Topology},
    set::{Collection, Poset},
  };
}

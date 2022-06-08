//! # Lib for cardano
//!
//! `lib` is a collection of utilities to make performing certain operations more convenient
pub mod git;
pub use git::*;
pub mod globals;
pub use globals::*;
pub mod os;
pub use os::*;
pub mod prereqs;
pub use prereqs::*;
pub mod setup;
pub use setup::*;

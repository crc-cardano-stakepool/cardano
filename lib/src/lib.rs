//! # Lib for cardano
//!
//! `lib` is a collection of utilities to make performing certain operations more convenient
pub mod utils;
pub use utils::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

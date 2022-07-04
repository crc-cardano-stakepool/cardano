//! # Lib for cardano
//!
//! `lib` is a collection of utilities to make performing certain operations more convenient
pub mod cli;
pub use cli::*;
pub mod utils;
pub use utils::*;
pub mod packages;
pub use packages::*;
pub mod configs;
pub use configs::*;
pub mod components;
pub use components::*;

#[cfg(test)]
mod test {
    #[test]
    fn test_network_info() {
        use cardano_multiplatform_lib::address::NetworkInfo;
        let testnet_info = NetworkInfo::testnet();
        log::debug!("{testnet_info:#?}");
        let mainnet_info = NetworkInfo::mainnet();
        log::debug!("{mainnet_info:#?}");
    }
}

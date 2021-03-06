pub const MIN_CORES: u8 = 2;
pub const MAINNET_MIN_FREE_DISK_SPACE_IN_GB: u8 = 75;
pub const MAINNET_RECOMMENDED_FREE_DISK_SPACE_IN_GB: u8 = 100;
pub const TESTNET_MIN_FREE_DISK_SPACE_IN_GB: u8 = 20;
pub const MAINNET_MIN_FREE_RAM_IN_GB: u8 = 16;
pub const TESTNET_MIN_FREE_RAM_IN_GB: u8 = 4;
pub const MIN_CPU_FREQUENCY_IN_MHZ: u16 = 1600;
pub const RECOMMENDED_CPU_FREQUENCY_IN_MHZ: u16 = 2000;
pub const GHC_VERSION: &str = "8.10.7";
pub const CABAL_VERSION: &str = "3.6.2.0";
pub const CARDANO_NODE_VERSION: &str = "1.35.0";
pub const LD_LIBRARY_PATH: &str = "/usr/local/lib:$LD_LIBRARY_PATH";
pub const PKG_CONFIG_PATH: &str = "/usr/local/lib/pkgconfig:$PKG_CONFIG_PATH";
pub const CARDANO_CONFIG_FILE_NAME: &str = "cardano.toml";
pub const GHCUP_URL: &str = "https://get-ghcup.haskell.org";
pub const CSNAPSHOT_BASE_URL: &str = "https://csnapshots.io";
pub const CSNAPSHOT_DOWNLOAD_URL: &str = "https://download.csnapshots.io";
pub const CSNAPSHOT_DATA_URL: &str = "https://data.csnapshots.io";
pub const SECP256K1_URL: &str = "https://github.com/bitcoin-core/secp256k1.git";
pub const LIBSODIUM_URL: &str =
    "https://github.com/input-output-hk/libsodium.git";
pub const CARDANO_URL: &str =
    "https://github.com/crc-cardano-stakepool/cardano.git";
pub const VERSIONS_URL: &str =
    "https://developers.cardano.org/docs/get-started/installing-cardano-node";
pub const CONFIG_BASE_URL: &str = "https://hydra.iohk.io/job/Cardano/cardano-node/cardano-deployment/latest-finished/download/1";
pub const DIRECTORIES: [&str; 11] = [
    "work",
    "ipc",
    "cardano",
    "config",
    "mainnet_config",
    "testnet_config",
    "mainnet_db",
    "testnet_db",
    "libsodium",
    "secp256k1",
    "install",
];

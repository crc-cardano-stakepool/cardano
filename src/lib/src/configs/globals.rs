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
pub const LD_LIBRARY_PATH: &str = "/usr/local/lib:$LD_LIBRARY_PATH";
pub const PKG_CONFIG_PATH: &str = "/usr/local/lib/pkgconfig:$PKG_CONFIG_PATH";
pub const CARDANO_CONFIG_FILE_NAME: &str = "cardano.toml";
pub const CARDANO_NODE_URL: &str = "https://github.com/input-output-hk/cardano-node.git";
pub const CARDANO_NODE_RELEASE_URL: &str = "https://api.github.com/repos/input-output-hk/cardano-node/releases/latest";
pub const LIBSODIUM_URL: &str = "https://github.com/input-output-hk/libsodium.git";
pub const SECP256K1_URL: &str = "https://github.com/bitcoin-core/secp256k1.git";
pub const CARDANO_URL: &str = "https://github.com/crc-cardano-stakepool/cardano.git";
pub const GHCUP_URL: &str = "https://get-ghcup.haskell.org";
pub const VERSIONS_URL: &str = "https://developers.cardano.org/docs/get-started/installing-cardano-node";
pub const SPINNERS: [&str; 6] = ["▹▹▹▹▹", "▸▹▹▹▹", "▹▸▹▹▹", "▹▹▸▹▹", "▹▹▹▸▹", "▹▹▹▹▸"];
pub const DIRECTORIES: [&str; 12] = [
    "work",
    "ipc",
    "cardano",
    "config",
    "mainnet_config",
    "testnet_config",
    "db",
    "mainnet_db",
    "testnet_db",
    "libsodium",
    "secp256k1",
    "install",
];
pub const DEBIAN_PACKAGES: [&str; 21] = [
    "curl",
    "automake",
    "build-essential",
    "pkg-config",
    "libffi-dev",
    "libgmp-dev",
    "libssl-dev",
    "libtinfo-dev",
    "libsystemd-dev",
    "zlib1g-dev",
    "libsodium-dev",
    "tidy",
    "make",
    "g++",
    "tmux",
    "git",
    "jq",
    "wget",
    "libncursesw5",
    "libtool",
    "autoconf",
];
pub const NON_DEBIAN_PACKAGES: [&str; 17] = [
    "curl",
    "git",
    "gcc",
    "gcc-c++",
    "tmux",
    "gmp-devel",
    "make",
    "tar",
    "xz",
    "wget",
    "zlib-devel",
    "tidy",
    "libtool",
    "autoconf",
    "systemd-devel",
    "ncurses-devel",
    "ncurses-compat-libs",
];

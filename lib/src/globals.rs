pub const GHC_VERSION: &str = "8.10.7";
pub const CABAL_VERSION: &str = "3.6.2.0";
pub const LD_LIBRARY_PATH: &str = "/usr/local/lib:$LD_LIBRARY_PATH";
pub const PKG_CONFIG_PATH: &str = "/usr/local/lib/pkgconfig:$PKG_CONFIG_PATH";
pub const CARDANO_NODE_URL: &str = "https://github.com/input-output-hk/cardano-node.git";
pub const LIBSODIUM_URL: &str = "https://github.com/input-output-hk/libsodium.git";
pub const SECP256K1_URL: &str = "https://github.com/bitcoin-core/secp256k1";
pub const GHCUP_URL: &str = "https://get-ghcup.haskell.org";
pub const VERSIONS_URL: &str = "https://developers.cardano.org/docs/get-started/installing-cardano-node";
pub const SPINNERS: [&str; 6] = ["▹▹▹▹▹", "▸▹▹▹▹", "▹▸▹▹▹", "▹▹▸▹▹", "▹▹▹▸▹", "▹▹▹▹▸"];
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

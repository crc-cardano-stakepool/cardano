use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref ENVS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("LD_LIBRARY_PATH", "/usr/local/lib:$LD_LIBRARY_PATH");
        map.insert("PKG_CONFIG_PATH", "/usr/local/lib/pkgconfig:$PKG_CONFIG_PATH");
        map
    };
    pub static ref PACKAGES: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert(
            "debian_packages",
            vec![
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
                "make",
                "g++",
                "tmux",
                "git",
                "jq",
                "wget",
                "libncursesw5",
                "libtool",
                "autoconf",
            ],
        );
        map.insert(
            "non_debian_packages",
            vec![
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
                "libtool",
                "autoconf",
                "systemd-devel",
                "ncurses-devel",
                "ncurses-compat-libs",
            ],
        );
        map
    };
    pub static ref PATHS: HashMap<&'static str, String> = {
        let ld = format!("export LD_LIBRARY_PATH={}", "\"/usr/local/lib:$LD_LIBRARY_PATH\"");
        let pkg = format!(
            "export PKG_CONFIG_PATH={}",
            "\"/usr/local/lib/pkgconfig:$PKG_CONFIG_PATH\""
        );
        let socket = format!(
            "export CARDANO_NODE_SOCKET_PATH={}",
            "\"$HOME/.cardano/ipc/node.socket\""
        );
        let local = format!("export PATH={}", "\"$HOME/.local/bin:$PATH\"");
        let cabal = format!("export PATH={}", "\"$HOME/.cabal/bin:$PATH\"");
        let ghcup = format!("export PATH={}", "\"$HOME/.ghcup/bin:$PATH\"");
        let mut map = HashMap::new();
        map.insert("LD_LIBRARY_PATH", ld);
        map.insert("PKG_CONFIG_PATH", pkg);
        map.insert("CARDANO_NODE_SOCKET_PATH", socket);
        map.insert(".local/bin", local);
        map.insert(".cabal/bin", cabal);
        map.insert(".ghcup/bin", ghcup);
        map
    };
    pub static ref SPINNERS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("arrows", vec!["▹▹▹▹▹", "▸▹▹▹▹", "▹▸▹▹▹", "▹▹▸▹▹", "▹▹▹▸▹", "▹▹▹▹▸"]);
        map
    };
    pub static ref URLS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("cardano-node", "https://github.com/input-output-hk/cardano-node.git");
        map.insert("libsodium", "https://github.com/input-output-hk/libsodium.git");
        map.insert("ghcup", "https://get-ghcup.haskell.org");
        map.insert(
            "versions",
            "https://developers.cardano.org/docs/get-started/installing-cardano-node",
        );
        map
    };
    pub static ref VERSIONS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("ghc", "8.10.7");
        map.insert("cabal", "3.4.0.0");
        map
    };
}

#[cfg(test)]
mod test {
    // use super::*;
    #[test]
    #[ignore]
    fn test_versions() {
        unimplemented!();
    }
    #[test]
    #[ignore]
    fn test_urls() {
        unimplemented!();
    }
    #[test]
    #[ignore]
    fn test_spinners() {
        unimplemented!();
    }
    #[test]
    #[ignore]
    fn test_paths() {
        unimplemented!();
    }
    #[test]
    #[ignore]
    fn test_packages() {
        unimplemented!();
    }
}

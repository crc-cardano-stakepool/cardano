use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
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
}

lazy_static! {
    pub static ref PATH_CONFIG: HashMap<&'static str, String> = {
        let mut map = HashMap::new();
        map.insert(
            "LD_LIBRARY_PATH",
            format!("export LD_LIBRARY_PATH={}", "\"/usr/local/lib:$LD_LIBRARY_PATH\""),
        );
        map.insert(
            "PKG_CONFIG_PATH",
            format!(
                "export PKG_CONFIG_PATH={}",
                "\"/usr/local/lib/pkgconfig:$PKG_CONFIG_PATH\""
            ),
        );
        map.insert(
            "CARDANO_NODE_SOCKET_PATH",
            format!(
                "export CARDANO_NODE_SOCKET_PATH={}",
                "\"$HOME/.cardano/ipc/node.socket\""
            ),
        );
        map.insert(".local/bin", format!("export PATH={}", "\"$HOME/.local/bin:$PATH\""));
        map.insert(".cabal/bin", format!("export PATH={}", "\"$HOME/.cabal/bin:$PATH\""));
        map.insert(".ghcup/bin", format!("export PATH={}", "\"$HOME/.ghcup/bin:$PATH\""));
        map
    };
}

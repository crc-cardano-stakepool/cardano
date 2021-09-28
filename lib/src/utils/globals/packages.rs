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

#[cfg(test)]
mod test {
    // use crate::PACKAGES;
    #[test]
    #[ignore]
    fn test_packages() {
        unimplemented!();
    }
}

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref URLS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert(
            "cardano-node",
            "https://github.com/input-output-hk/cardano-node.git",
        );
        map.insert(
            "libsodium",
            "https://github.com/input-output-hk/libsodium.git",
        );
        map.insert("ghcup", "https://get-ghcup.haskell.org");
        map
    };
}

#[cfg(test)]
mod test {
    // use crate::URLS;
    #[test]
    #[ignore]
    fn test_urls() {
        unimplemented!();
    }
}

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref VERSIONS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("ghc", "8.10.4");
        map.insert("cabal", "3.4.0.0");
        map
    };
}

#[cfg(test)]
mod test {
    // use crate::VERSIONS;
    #[test]
    #[ignore]
    fn test_versions() {
        unimplemented!();
    }
}

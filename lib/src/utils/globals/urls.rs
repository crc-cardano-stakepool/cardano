use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref URLS: HashMap<&'static str, String> = {
        let mut map = HashMap::new();
        map.insert(
            "cardano-node",
            "https://github.com/input-output-hk/cardano-node.git".to_string(),
        );
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

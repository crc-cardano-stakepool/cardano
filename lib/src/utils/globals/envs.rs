use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref ENVS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("LD_LIBRARY_PATH", "/usr/local/lib:$LD_LIBRARY_PATH");
        map.insert(
            "PKG_CONFIG_PATH",
            "/usr/local/lib/pkgconfig:$PKG_CONFIG_PATH",
        );
        map.insert("PATH", "$HOME/.local/bin:$PATH");
        map
    };
}

#[cfg(test)]
mod test {
    // use crate::PATHS;
    #[test]
    #[ignore]
    fn test_paths() {
        unimplemented!();
    }
}

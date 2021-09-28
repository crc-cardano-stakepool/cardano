use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
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

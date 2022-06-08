use crate::URLS;

pub fn get_ghc_version_url() -> &'static str {
    if let Some(url) = URLS.get("versions") {
        url
    } else {
        "https://developers.cardano.org/docs/get-started/installing-cardano-node"
    }
}

#[cfg(test)]
mod test {
    // use crate::get_ghcup_install_url;
    #[test]
    #[ignore]
    fn test_get_ghcup_install_url() {
        unimplemented!();
    }
}

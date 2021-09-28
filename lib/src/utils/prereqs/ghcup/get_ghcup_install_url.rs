use crate::URLS;

pub fn get_ghcup_install_url() -> &'static str {
    if let Some(url) = URLS.get("ghcup") {
        url
    } else {
        "https://get-ghcup.haskell.org"
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

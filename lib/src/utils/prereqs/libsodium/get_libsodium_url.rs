use crate::URLS;

pub fn get_libsodium_url() -> &'static str {
    if let Some(url) = URLS.get("libsodium") {
        url
    } else {
        "https://github.com/input-output-hk/libsodium.git"
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

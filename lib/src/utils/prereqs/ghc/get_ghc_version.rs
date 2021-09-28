use crate::VERSIONS;

pub fn get_ghc_version() -> &'static str {
    if let Some(version) = VERSIONS.get("ghc") {
        version
    } else {
        "8.10.4"
    }
}

#[cfg(test)]
mod test {
    // use crate::get_ghc_version;
    #[test]
    #[ignore]
    fn test_get_ghc_version() {
        unimplemented!();
    }
}

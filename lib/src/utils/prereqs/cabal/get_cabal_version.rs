use crate::VERSIONS;

pub fn get_cabal_version() -> &'static str {
    if let Some(version) = VERSIONS.get("cabal") {
        version
    } else {
        "3.4.0.0"
    }
}

#[cfg(test)]
mod test {
    // use crate::get_cabal_version;
    #[test]
    #[ignore]
    fn test_get_cabal_version() {
        unimplemented!();
    }
}

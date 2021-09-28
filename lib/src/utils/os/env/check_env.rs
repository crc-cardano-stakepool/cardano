use anyhow::{anyhow, Result};
use std::env::var;

pub fn check_env(key: &str) -> Result<String> {
    match var(key) {
        Ok(val) => Ok(val),
        Err(e) => Err(anyhow!("couldn't interpret {}: {}", key, e)),
    }
}

#[cfg(test)]
mod test {
    // use crate::check_env;
    #[test]
    #[ignore]
    fn test_check_env() {
        unimplemented!();
    }
}

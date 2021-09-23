use anyhow::{anyhow, Result};
use std::env;

pub fn check_env(key: &str) -> Result<String> {
    match env::var(key) {
        Ok(val) => Ok(val),
        Err(e) => Err(anyhow!("couldn't interpret {}: {}", key, e)),
    }
}

pub fn set_env(key: &str, value: &str) {
    env::set_var(key, value);
}

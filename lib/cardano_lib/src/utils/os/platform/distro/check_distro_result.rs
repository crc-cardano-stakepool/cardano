use crate::print;
use anyhow::{anyhow, Result};

pub fn check_distro_result(distro: Result<String>) -> Result<String> {
    match distro {
        Ok(result) => {
            let msg = format!("Detected {}", result.trim());
            print("green", &msg)?;
            Ok(result)
        }
        Err(e) => Err(anyhow!("{}", e)),
    }
}

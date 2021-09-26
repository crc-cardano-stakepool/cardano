use anyhow::Result;
use sudo::{check, RunningAs};

pub fn check_root() -> Result<bool> {
    if let RunningAs::Root = check() {
        Ok(true)
    } else {
        Ok(false)
    }
}

use anyhow::Result;
use sudo::{check, RunningAs};

pub fn check_root() -> Result<bool> {
    if let RunningAs::Root = check() {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[cfg(test)]
mod test {
    // use crate::check_root;
    #[tokio::test]
    #[ignore]
    async fn test_check_root() {
        unimplemented!();
    }
}

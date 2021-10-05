use crate::{create_dir, print};
use anyhow::Result;
use std::path::Path;

pub async fn check_dir(absolute_path: &str) -> Result<()> {
    if !Path::new(absolute_path).is_dir() {
        let msg = format!("{} does not exist", absolute_path);
        print("yellow", &msg)?;
        create_dir(absolute_path).await
    } else {
        let msg = format!("{} directory found", absolute_path);
        print("green", &msg)
    }
}

#[cfg(test)]
mod test {
    // use crate::check_dir;
    #[tokio::test]
    #[ignore]
    async fn test_check_dir() {
        unimplemented!();
    }
}

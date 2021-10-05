use crate::{create_dir, print};
use anyhow::Result;
use std::path::Path;

pub async fn check_dir(absolute_path: &str) -> Result<()> {
    if !Path::new(absolute_path).is_dir() {
        create_dir(absolute_path).await
    } else {
        let msg = format!("{} is not a directory or does not exist", absolute_path);
        print("red", &msg)
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

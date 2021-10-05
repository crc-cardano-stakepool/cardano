use crate::chownr;
use anyhow::Result;
use tokio::fs::create_dir_all;

pub async fn create_dir(absolute_path: &str) -> Result<()> {
    create_dir_all(absolute_path).await?;
    chownr(absolute_path).await
}

#[cfg(test)]
mod test {
    // use crate::create_dir;
    #[tokio::test]
    #[ignore]
    async fn test_create_dir() {
        unimplemented!();
    }
}

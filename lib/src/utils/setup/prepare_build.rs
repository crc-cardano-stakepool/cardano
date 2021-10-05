use crate::{check_dependencies, check_dir, check_work_dir, print, setup_packages, setup_shell, setup_work_dir};
use anyhow::Result;

pub async fn prepare_build() -> Result<()> {
    print("", "Preparing build")?;
    check_dir(&check_work_dir().await?).await?;
    setup_packages().await?;
    setup_shell().await?;
    setup_work_dir().await?;
    check_dependencies().await
}

#[cfg(test)]
mod test {
    // use crate::prepare_build;
    #[tokio::test]
    #[ignore]
    async fn test_prepare_build() {
        unimplemented!();
    }
}

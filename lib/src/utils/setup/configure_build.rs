use crate::print;
use anyhow::Result;

pub async fn configure_build(component: &str) -> Result<()> {
    let msg = format!("Configuring the build of {}", component);
    print("", &msg)?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::configure_build;
    #[tokio::test]
    #[ignore]
    async fn test_configure_build() {
        unimplemented!();
    }
}

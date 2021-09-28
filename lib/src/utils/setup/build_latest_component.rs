use crate::print;
use anyhow::Result;

pub async fn build_latest_component(component: &str) -> Result<()> {
    let msg = format!("Building {}", component);
    print("", &msg)?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::build_latest_component;
    #[tokio::test]
    #[ignore]
    async fn test_build_latest_component() {
        unimplemented!();
    }
}

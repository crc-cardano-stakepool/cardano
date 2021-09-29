use crate::print;
use anyhow::Result;

pub async fn check_component(component: &str) -> Result<()> {
    let msg = format!("Checking {} installation", component);
    print("", &msg)?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::check_component;
    #[tokio::test]
    #[ignore]
    async fn test_check_component() {
        unimplemented!();
    }
}

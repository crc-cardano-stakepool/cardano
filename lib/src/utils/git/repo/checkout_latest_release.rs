use crate::print;
use anyhow::Result;

pub async fn checkout_latest_release(component: &str) -> Result<()> {
    let msg = format!("Checkout latest {} version", component);
    print("", &msg)?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::checkout_latest_release;
    #[tokio::test]
    #[ignore]
    async fn test_checkout_latest_release() {
        unimplemented!();
    }
}

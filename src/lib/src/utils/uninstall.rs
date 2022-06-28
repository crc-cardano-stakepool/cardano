use anyhow::Result;

pub async fn uninstall_component(component: &str) -> Result<()> {
    if component == "cardano-node" {
        println!("Uninstalling cardano-node")
    }
    Ok(())
}

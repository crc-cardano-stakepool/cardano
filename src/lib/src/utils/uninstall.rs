use anyhow::Result;

pub async fn uninstall_component(component: &str) -> Result<()> {
    match component {
        "cardano-node" => println!("Uninstalling cardano-node"),
        _ => (),
    }
    Ok(())
}

use crate::{async_command, check_env, get_component_path, print, set_env};
use anyhow::{anyhow, Result};

pub async fn copy_binary(component: &str) -> Result<()> {
    let install_dir = check_env("INSTALL_DIR")?;
    let msg = format!("Copying {} binary to {}", component, install_dir);
    print("", &msg)?;
    match component {
        "cardano-node" => {
            let path = get_component_path(component).await?;
            let bin_path = format!("{}/scripts/bin-path.sh", path);
            let node = format!("cd {} && cp -p \"$({} cardano-node)\" {}", path, bin_path, install_dir);
            let cli = format!("cd {} && cp -p \"$({} cardano-cli)\" {}", path, bin_path, install_dir);
            println!("{}\n{}", node, cli);
            async_command(&node).await?;
            async_command(&cli).await?;
            let node_bin = format!("{}/cardano-node", install_dir);
            let cli_bin = format!("{}/cardano-cli", install_dir);
            set_env("CARDANO_NODE_BIN", &node_bin);
            set_env("CARDANO_CLI_BIN", &cli_bin);
            let msg = format!("Successfully copied binaries to {}", install_dir);
            print("", &msg)?;
            Ok(())
        }
        _ => Err(anyhow!("Unknown component")),
    }
}

#[cfg(test)]
mod test {
    // use crate::copy_binary;
    #[tokio::test]
    #[ignore]
    async fn test_copy_binary() {
        unimplemented!();
    }
}

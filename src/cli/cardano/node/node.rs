use super::run::RunCommand;
use crate::cli::utils::terminal::Terminal;
use anyhow::Result;
use console::Emoji;
use reqwest::Client;
use serde_json::{json, Value};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Manage cardano nodes")]
pub enum NodeCommand {
    Run(RunCommand),
    #[structopt(about = "Install the latest cardano-node binary")]
    Install,
    #[structopt(about = "Uninstalls the cardano-node")]
    Uninstall,
}

impl NodeCommand {
    pub async fn exec(cmd: NodeCommand) -> Result<()> {
        match cmd {
            NodeCommand::Run(cmd) => RunCommand::exec(cmd).await?,
            NodeCommand::Install => NodeCommand::install_node().await?,
            NodeCommand::Uninstall => NodeCommand::uninstall_node().await?,
        }
        Ok(())
    }

    pub async fn check_node_version() -> Result<bool> {
        let latest_node_version = NodeCommand::fetch_latest_node_version().await?;
        let fetch_node_version = "cardano-node --version | awk '{print $2}'| head -n1";
        let node_version = Terminal::async_command(&fetch_node_version).await?;
        let installed_node_version = node_version.trim();
        if NodeCommand::compare_latest_node_version(&installed_node_version, &latest_node_version).await? {
            Ok(true)
        } else {
            Terminal::print("red", "Cardano node is not installed", Emoji("😔", ""))?;
            Ok(false)
        }
    }

    pub async fn compare_latest_node_version(installed_node_version: &str, latest_node_version: &str) -> Result<bool> {
        if latest_node_version.eq(installed_node_version) {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn fetch_latest_node_version() -> Result<String> {
        const RELEASE_URL: &str = "https://api.github.com/repos/input-output-hk/cardano-node/releases/latest";
        let client = Client::new();
        let response: Value = client.get(RELEASE_URL).header("User-Agent", "Web 3").send().await?.json().await?;
        let latest_node_version = json!(response)["tag_name"].to_string().trim().replace("\"", "");
        Ok(latest_node_version)
    }

    pub async fn install_node() -> Result<()> {
        if !NodeCommand::check_node_version().await? {
            if Terminal::proceed("Do you want to install the latest cardano-node binary?")? {
                Terminal::print("white", "Installing latest cardano node", Emoji("🤟", ""))?;
            } else {
                Terminal::print("red", "Aborted cardano-node installation", Emoji("😔", ""))?;
            }
        } else {
            Terminal::print("green", "The latest cardano node version is installed", Emoji("🙌🎉", ""))?;
        }
        Ok(())
    }

    pub async fn uninstall_node() -> Result<()> {
        Terminal::print("white", "Uninstalling cardano-node", Emoji("💔", ""))?;
        Ok(())
    }
}

#[cfg(test)]
mod test {

    #[test]
    #[ignore]
    fn test_check_node_version() {
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn test_install_node() {
        unimplemented!();
    }
}

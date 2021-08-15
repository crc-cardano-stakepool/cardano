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
}

impl NodeCommand {
    pub async fn exec(cmd: NodeCommand) -> Result<()> {
        match cmd {
            NodeCommand::Run(cmd) => RunCommand::exec(cmd).await?,
            NodeCommand::Install => NodeCommand::install_node().await?,
        }
        Ok(())
    }

    pub async fn check_node_version() -> Result<bool> {
        let fetch_node_version = "cardano-node --version | awk '{print $2}'| head -n1";
        if let Ok(node_version) = Terminal::async_command(&fetch_node_version).await {
            let version = node_version.trim();
            match NodeCommand::compare_latest_node_version(version).await {
                Ok(true) => {
                    Terminal::print("white", "", Emoji("", ""))?;
                    let up_to_date = format!("The latest cardano node version (v{}) is already installed", version);
                    Terminal::print("green", &up_to_date, Emoji("", ""))?;
                    Ok(true)
                }
                Ok(false) => {
                    Terminal::print("red", "Cardano node is not installed", Emoji("", ""))?;
                    Ok(false)
                }
                Err(e) => {
                    panic!("{}", e)
                }
            }
        } else {
            Ok(false)
        }
    }

    pub async fn compare_latest_node_version(installed_node_version: &str) -> Result<bool> {
        let latest_node_version = NodeCommand::fetch_latest_node_version().await;
        if let Ok(latest_node_version) = latest_node_version {
            if latest_node_version.eq(installed_node_version) {
                Ok(true)
            } else {
                Ok(false)
            }
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
        if let Ok(false) = NodeCommand::check_node_version().await {
            if let Ok(true) = Terminal::proceed("Do you want to install the latest cardano-node binary?") {
                Terminal::print("white", "Installing latest cardano node", Emoji("", ""))?;
            } else {
                Terminal::print("red", "Aborted cardano-node installation", Emoji("", ""))?;
            }
        }
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

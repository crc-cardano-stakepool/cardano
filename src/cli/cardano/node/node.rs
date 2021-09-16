use super::run::RunCommand;
use crate::cli::utils::{async_command, check_directory, check_install_dir, check_root, print, proceed};
use anyhow::Result;
use console::Emoji;
use reqwest::Client;
use serde_json::{json, Value};
use structopt::StructOpt;
use sudo::escalate_if_needed;

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

    pub async fn install_node() -> Result<()> {
        if let Ok(false) = check_root() {
            print("", "Checking for root privileges", Emoji("", ""))?;
            match escalate_if_needed() {
                Ok(user) => println!("Running as {:#?}", user),
                Err(_) => println!("Failed obtaining root privileges"),
            }
        } else {
            if !NodeCommand::check_node_version().await? {
                if proceed("Do you want to install the latest cardano-node binary?")? {
                    print("white", "Installing latest cardano node", Emoji("🤟", ""))?;
                    let default_directory = String::from("~/.cardano");
                    let install_dir = check_install_dir().await?;
                    match install_dir.as_str() {
                        "~/.cardano" => {
                            check_directory("default directory", &default_directory).await?;
                        }
                        _ => println!("Custom directory"),
                    }
                } else {
                    print("red", "Aborted cardano-node installation", Emoji("😔", ""))?;
                }
            } else {
                print("green", "The latest cardano node version is installed", Emoji("🙌🎉", ""))?;
            }
        }
        Ok(())
    }

    pub async fn check_node_version() -> Result<bool> {
        let latest_node_version: String = NodeCommand::fetch_latest_node_version().await?;
        let installed_node_version: String = NodeCommand::fetch_installed_node_version().await?;
        if NodeCommand::compare_latest_node_version(&installed_node_version, &latest_node_version).await? {
            Ok(true)
        } else {
            print("red", "Cardano node is not installed", Emoji("😔", ""))?;
            Ok(false)
        }
    }

    pub async fn fetch_installed_node_version() -> Result<String> {
        let fetch_node_version = "cardano-node --version | awk '{print $2}'| head -n1";
        let node_version: String = async_command(&fetch_node_version).await?;
        let installed_node_version: String = String::from(node_version.trim());
        Ok(installed_node_version)
    }

    pub async fn fetch_latest_node_version() -> Result<String> {
        const RELEASE_URL: &str = "https://api.github.com/repos/input-output-hk/cardano-node/releases/latest";
        let client = Client::new();
        let response: Value = client.get(RELEASE_URL).header("User-Agent", "Web 3").send().await?.json().await?;
        let latest_node_version: String = json!(response)["tag_name"].to_string().trim().replace("\"", "");
        Ok(latest_node_version)
    }

    pub async fn compare_latest_node_version(installed_node_version: &str, latest_node_version: &str) -> Result<bool> {
        if installed_node_version.eq(latest_node_version) {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn uninstall_node() -> Result<()> {
        print("white", "Uninstalling cardano-node", Emoji("💔", ""))?;
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
    fn test_fetch_latest_node_version() {
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn test_fetch_installed_node_version() {
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn test_compare_latest_node_version() {
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn test_install_node() {
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn test_uninstall_node() {
        unimplemented!();
    }
}

use super::run::RunCommand;
use crate::cli::utils::{async_command_pipe, check_directory, check_root, check_user, print, proceed, update_os_packages};
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
            match escalate_if_needed() {
                Ok(user) => println!("Running as {:#?}", user),
                Err(_) => println!("Failed obtaining root privileges"),
            }
        } else {
            if !NodeCommand::check_node_version().await? {
                if proceed("Do you want to install the latest cardano-node binary?")? {
                    let user = check_user().await?;
                    let install_directory: String = format!("/home/{}/.cardano", user.trim());
                    print("white", "Installing latest cardano node", Emoji("🤟", ""))?;
                    check_directory("install directory", &install_directory).await?;
                    update_os_packages().await?;
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
        let latest_node_version: String = NodeCommand::check_latest_node_version().await?;
        let installed_node_version: String = NodeCommand::check_installed_version("cardano-node").await?;
        if NodeCommand::compare_latest_node_version(&installed_node_version, &latest_node_version).await? {
            Ok(true)
        } else {
            print("red", "Cardano node is not installed", Emoji("😔", ""))?;
            Ok(false)
        }
    }

    pub async fn check_installed_version(component: &str) -> Result<String> {
        let cmd = format!("type {}", component);
        let installed = String::from(async_command_pipe(&cmd).await?);
        if !installed.contains("not found") {
            let helper_string = "'{print $2}'";
            let cmd = format!("{} --version | awk {} | head -n1", component, helper_string);
            let version: String = async_command_pipe(&cmd).await?;
            let installed_version: String = String::from(version.trim());
            Ok(installed_version)
        } else {
            Ok(String::from("not found"))
        }
    }

    pub async fn check_latest_node_version() -> Result<String> {
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

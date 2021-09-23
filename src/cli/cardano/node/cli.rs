use super::run::RunCommand;
use crate::cli::utils::check_version::check_version;
use crate::cli::utils::color::print;
use crate::cli::utils::dialog::proceed;
use crate::cli::utils::fs::{check_directory, check_work_dir};
use crate::cli::utils::os::*;
use crate::cli::utils::shell::setup_shell;
use crate::cli::utils::user::*;
use anyhow::Result;
use console::Emoji;
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
        } else if !check_version("cardano-node").await? {
            if proceed("Do you want to install the latest cardano-node binary?")? {
                print("white", "Installing latest cardano node", Emoji("🤟", ""))?;
                check_directory("install directory", &check_work_dir().await?).await?;
                setup_packages().await?;
                setup_shell().await?;
            } else {
                print("red", "Aborted cardano-node installation", Emoji("😔", ""))?;
            }
        } else {
            print(
                "green",
                "The latest cardano node version is installed",
                Emoji("🙌🎉", ""),
            )?;
        }
        Ok(())
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
    fn test_install_node() {
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn test_uninstall_node() {
        unimplemented!();
    }
}

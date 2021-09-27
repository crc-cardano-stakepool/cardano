use super::run::RunCommand;
use anyhow::Result;
use cardano_lib::{
    check_dir, check_root, check_version, check_work_dir, clone_cardano_repo, prepare_build, print_emoji, proceed,
    setup_packages, setup_shell,
};
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
                print_emoji("white", "Installing latest cardano node", Emoji("🤟", ""))?;
                check_dir(&check_work_dir().await?).await?;
                setup_packages().await?;
                setup_shell().await?;
                prepare_build().await?;
                clone_cardano_repo("cardano-node").await?;
            } else {
                print_emoji("red", "Aborted cardano-node installation", Emoji("😔", ""))?;
            }
        } else {
            print_emoji(
                "green",
                "The latest cardano node version is installed",
                Emoji("🙌🎉", ""),
            )?;
        }
        Ok(())
    }

    pub async fn uninstall_node() -> Result<()> {
        print_emoji("white", "Uninstalling cardano-node", Emoji("💔", ""))?;
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

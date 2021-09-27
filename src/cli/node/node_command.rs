use super::run::RunCommand;
use anyhow::Result;
use console::Emoji;
use lib::{install_component, print_emoji};
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

    pub async fn install_node() -> Result<()> {
        install_component("cardano-node").await?;
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

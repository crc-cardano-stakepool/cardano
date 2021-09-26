use super::RunConfig;
use anyhow::Result;
use console::Emoji;
use lib::{check_version, print, print_emoji};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Run cardano node")]
pub enum RunCommand {
    #[structopt(about = "Run cardano node in mainnet")]
    Mainnet(RunConfig),
    #[structopt(about = "Run cardano node in testnet")]
    Testnet(RunConfig),
}

impl RunCommand {
    pub async fn exec(cmd: RunCommand) -> Result<()> {
        match cmd {
            RunCommand::Mainnet(config) => RunCommand::mainnet(config).await?,
            RunCommand::Testnet(config) => RunCommand::testnet(config).await?,
        }
        Ok(())
    }

    async fn mainnet(config: RunConfig) -> Result<()> {
        let output: String = format!("The config to run node in mainnet: {:#?}", config);
        print("white", &output)?;
        if check_version("cardano-node").await? {
            print_emoji("green", "Proceeding to run node in mainnet", Emoji("💪", ""))?;
        }
        Ok(())
    }

    async fn testnet(config: RunConfig) -> Result<()> {
        let output: String = format!("The config to run node in testnet: {:#?}", config);
        print("white", &output)?;
        if check_version("cardano-node").await? {
            print_emoji("green", "Proceeding to run node in testnet", Emoji("🔍", ""))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    #[test]
    #[ignore]
    fn test_mainnet() {
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn test_testnet() {
        unimplemented!();
    }
}
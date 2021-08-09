use std::path::PathBuf;
use structopt::StructOpt;
use std::net::IpAddr;

#[derive(Debug, StructOpt)]
#[structopt(about = "Manage cardano nodes")]
pub enum NodeCommand {
    Run {
        #[structopt(subcommand, help = "The run configuration for mainnet")]
        network: RunCommand,
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "run", about = "Run cardano node in mainnet or testnet")]
pub enum RunCommand {
    Mainnet {
        #[structopt(subcommand, about = "The run configuration for mainnet")]
        run: MainnetCommand,
    },
    Testnet {
        #[structopt(subcommand, about = "The run configuration for testnet")]
        run: TestnetCommand,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(name = "mainnet", about = "Run cardano node in mainnet")]
pub struct MainnetCommand {
    #[structopt(subcommand)]
    run: Option<RunConfig>
}

#[derive(Debug, StructOpt)]
#[structopt(name = "testnet", about = "Run cardano node in testnet")]
pub struct TestnetCommand {
    #[structopt(subcommand)]
    run: Option<RunConfig>
}

#[derive(Debug, StructOpt)]
pub struct RunConfig {
    #[structopt(short, long, default_value = "3001")]
    port: u16,
    #[structopt(short, long, parse(from_os_str))]
    db: PathBuf,
    #[structopt(short, long, default_value = "127.0.0.1")]
    host: IpAddr,  
    #[structopt(short, long, parse(from_os_str))]
    topology: PathBuf,
    #[structopt(short, long, parse(from_os_str))]
    socket: PathBuf,
    #[structopt(short, long, parse(from_os_str))]
    config: PathBuf
}
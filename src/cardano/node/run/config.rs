use std::net::IpAddr;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "mainnet", about = "Run cardano node in mainnet")]
pub struct RunConfig {
    #[structopt(short, default_value = "3001")]
    port: u16,
    #[structopt(short, parse(from_os_str))]
    db: PathBuf,
    #[structopt(short, default_value = "127.0.0.1")]
    host: IpAddr,
    #[structopt(short, parse(from_os_str))]
    topology: PathBuf,
    #[structopt(short, parse(from_os_str))]
    socket: PathBuf,
    #[structopt(short, parse(from_os_str))]
    config: PathBuf,
}

impl RunConfig {
    pub fn mainnet(config: RunConfig) {
        println!("The config to run node in mainnet: {:#?}", config)
    }

    pub fn testnet(config: RunConfig) {
        println!("The config to run node in testnet: {:#?}", config)
    }
}

use clap::Args;
use std::net::IpAddr;
use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Debug, Args)]
pub struct RunConfig {
    #[clap(value_parser)]
    port: u16,
    #[clap(value_parser)]
    db: PathBuf,
    #[clap(value_parser)]
    host: IpAddr,
    #[clap(value_parser)]
    topology: PathBuf,
    #[clap(value_parser)]
    socket: PathBuf,
    #[clap(value_parser)]
    config: PathBuf,
}

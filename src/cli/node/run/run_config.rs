use clap_verbosity_flag::Verbosity;
use std::net::IpAddr;
use std::path::PathBuf;
use structopt::StructOpt;

#[allow(dead_code)]
#[derive(Debug, StructOpt)]
pub struct RunConfig {
    #[structopt(
        short,
        default_value = "3001",
        about = "The port the node runs on"
    )]
    port: u16,
    #[structopt(
        short,
        parse(from_os_str),
        about = "The path to the blockchain"
    )]
    db: PathBuf,
    #[structopt(
        short,
        default_value = "127.0.0.1",
        about = "The IP on which the node runs"
    )]
    host: IpAddr,
    #[structopt(
        short,
        parse(from_os_str),
        about = "The path to the networking topology configurtion file between nodes and relays"
    )]
    topology: PathBuf,
    #[structopt(
        short,
        parse(from_os_str),
        about = "The path to the socket for inter-process-communication between cardano components"
    )]
    socket: PathBuf,
    #[structopt(
        short,
        parse(from_os_str),
        about = "The path to the configuration file for the node"
    )]
    config: PathBuf,
    #[structopt(flatten)]
    verbose: Verbosity,
}

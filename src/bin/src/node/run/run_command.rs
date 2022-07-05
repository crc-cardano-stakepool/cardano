use anyhow::Result;
use clap::{Args, Subcommand};
use lib::{
    check_config_files, handle_config, handle_db, handle_socket, handle_topology, match_network, parse_config_to_command, proceed,
    run_node_if_installed, SystemRequirements,
};
use std::{
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
};

#[derive(Debug, Args)]
pub struct Run {
    #[clap(subcommand)]
    command: RunCommand,
}

#[derive(Debug, Subcommand)]
pub enum RunCommand {
    /// Run cardano-node in mainnet
    Mainnet(RunArgs),
    /// Run cardano-node in testnet
    Testnet(RunArgs),
}

#[derive(Debug, Args)]
pub struct RunArgs {
    /// The port on which the node will be listening to incoming connections
    #[clap(long, value_parser = clap::value_parser!(u16).range(1..), default_value_t = 3001)]
    pub port: u16,
    /// The IP of the node, other relays will connect to this IP
    #[clap(long, value_parser, default_value_t = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))]
    pub host: IpAddr,
    /// Points to the socket that is used by the components for inter process communication
    #[clap(long, parse(from_os_str))]
    pub socket: Option<PathBuf>,
    /// Points to the path where the blockchain will be stored
    #[clap(long, parse(from_os_str))]
    pub db: Option<PathBuf>,
    /// Points to the topology.json which contains a list of network peers that the node will connect to
    #[clap(long, parse(from_os_str))]
    pub topology: Option<PathBuf>,
    /// Points to the config.json which contains general node settings i.e. logging and versioning
    #[clap(long, parse(from_os_str))]
    pub config: Option<PathBuf>,
}

impl RunCommand {
    pub async fn exec(cmd: Run) -> Result<()> {
        match cmd.command {
            RunCommand::Mainnet(config) => RunCommand::run_node(config, "mainnet").await,
            RunCommand::Testnet(config) => RunCommand::run_node(config, "testnet").await,
        }
    }
    pub async fn run_node(config: RunArgs, network: &str) -> Result<()> {
        if !SystemRequirements::check_requirements() && !proceed("Do you still want to run the node anyway?")? {
            return Ok(());
        }
        let network = match_network(network);
        check_config_files(network).await?;
        let port = config.port;
        let host = config.host;
        let socket = handle_socket(config.socket)?;
        let db = handle_db(config.db, network)?;
        let topology = handle_topology(config.topology, network)?;
        let config = handle_config(config.config, network)?;
        let cmd = parse_config_to_command(port, host, config, &db, socket, topology);
        run_node_if_installed(&cmd, network, db).await
    }
}

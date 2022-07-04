use anyhow::Result;
use cardano_multiplatform_lib::NetworkIdKind;
use clap::{Args, Subcommand};
use lib::{is_bin_installed, path_to_string, read_setting};
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
    #[clap(long, value_parser = clap::value_parser!(u16).range(1..), default_value_t = 3001)]
    pub port: u16,
    #[clap(long, parse(from_os_str))]
    pub db: Option<PathBuf>,
    #[clap(long, value_parser, default_value_t = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))]
    pub host: IpAddr,
    #[clap(long, parse(from_os_str))]
    pub topology: Option<PathBuf>,
    #[clap(long, parse(from_os_str))]
    pub socket: Option<PathBuf>,
    #[clap(long, parse(from_os_str))]
    pub config: Option<PathBuf>,
}

impl RunCommand {
    pub async fn exec(cmd: Run) -> Result<()> {
        match cmd.command {
            RunCommand::Mainnet(config) => RunCommand::run_node(config, NetworkIdKind::Mainnet).await,
            RunCommand::Testnet(config) => RunCommand::run_node(config, NetworkIdKind::Testnet).await,
        }
    }
    pub async fn run_node(mut config: RunArgs, network: NetworkIdKind) -> Result<()> {
        let net: &str = match network {
            NetworkIdKind::Mainnet => "mainnet",
            NetworkIdKind::Testnet => "testnet",
        };
        log::debug!("The config to run node in {net}: {config:#?}");
        if config.db.is_none() {
            config.db = Self::get_db(net)?;
        }
        if config.topology.is_none() {
            config.topology = Self::get_topology(net)?;
        }
        if config.socket.is_none() {
            let setting = read_setting("ipc_dir")?;
            let mut socket = PathBuf::from(&setting);
            socket.push("node.socket");
            config.socket = Some(socket);
        }
        if config.config.is_none() {
            config.config = Self::get_config(net)?;
        }
        let command = RunCommand::parse_config_to_command(config);
        log::debug!("The command to run a cardano node in {net}: {command}");
        if is_bin_installed("cardano-node").await? {
            log::info!("Proceeding to run node in {net}");
        } else {
            log::error!("cardano-node is not installed");
        }
        Ok(())
    }
    pub fn get_db(network: &str) -> Result<Option<PathBuf>> {
        let key = format!("{network}_db_dir");
        let setting = read_setting(&key)?;
        let db = PathBuf::from(&setting);
        Ok(Some(db))
    }
    pub fn get_topology(network: &str) -> Result<Option<PathBuf>> {
        let key = format!("{network}_db_dir");
        let setting = read_setting(&key)?;
        let mut topology = PathBuf::from(&setting);
        let key = format!("{network}-topology.json");
        topology.push(key);
        Ok(Some(topology))
    }
    pub fn get_config(network: &str) -> Result<Option<PathBuf>> {
        let key = format!("{network}_config_dir");
        let setting = read_setting(&key)?;
        let mut config_dir = PathBuf::from(&setting);
        let key = format!("{network}-config.json");
        config_dir.push(key);
        Ok(Some(config_dir))
    }
    pub fn parse_config_to_command(config: RunArgs) -> String {
        log::debug!("The parsed config to run node in testnet: {config:#?}");
        let port = config.port;
        let host = config.host;
        let net_config = path_to_string(&config.config.unwrap()).unwrap();
        let db = path_to_string(&config.db.unwrap()).unwrap();
        let socket = path_to_string(&config.socket.unwrap()).unwrap();
        let topology = path_to_string(&config.topology.unwrap()).unwrap();
        let command = format!(
            "cardano-node run \
            --topology {topology} \
            --database-path {db} \
            --socket-path {socket} \
            --host-addr {host} \
            --port {port} \
            --config {net_config} \
            "
        );
        command
    }
}

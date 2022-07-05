use anyhow::{anyhow, Result};
use cardano_multiplatform_lib::NetworkIdKind;
use clap::{Args, Subcommand};
use lib::{
    async_command, check_config_files, check_installed_version, check_latest_version, download_snapshot, get_config, get_db, get_topology,
    install_component, is_bin_installed, path_to_string, proceed, read_setting,
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
        check_config_files(net).await?;
        if config.db.is_none() {
            config.db = get_db(net)?;
        } else {
            let path = config.db.as_ref().unwrap();
            if path.is_file() {
                log::error!("Invalid db argument");
                return Err(anyhow!("Database path can not be a file"));
            }
            if !path.is_absolute() {
                log::error!("Invalid db argument");
                let path = path_to_string(path)?;
                return Err(anyhow!("Not an absolute path: {path}"));
            }
            if !path.exists() {
                log::error!("Invalid db argument");
                let path = path_to_string(path)?;
                return Err(anyhow!("Invalid path does not exist: {path}"));
            }
        }
        if config.topology.is_none() {
            config.topology = get_topology(net)?;
        } else {
            let path = config.topology.as_ref().unwrap();
            if !path.is_dir() {
                log::error!("Invalid topology argument");
                return Err(anyhow!("Topology path can not be a file"));
            }
            if !path.is_absolute() {
                log::error!("Invalid topology argument");
                let path = path_to_string(path)?;
                return Err(anyhow!("Not an absolute path: {path}"));
            }
            if !path.exists() {
                log::error!("Invalid topology argument");
                let path = path_to_string(path)?;
                return Err(anyhow!("Invalid path does not exist: {path}"));
            }
        }
        if config.socket.is_none() {
            let path = read_setting("ipc_dir")?;
            let mut socket = PathBuf::from(&path);
            if !socket.exists() {
                log::error!("Invalid socket");
                return Err(anyhow!("The path {path} does not exist"));
            }
            socket.push("node.socket");
            config.socket = Some(socket);
        } else {
            let mut path = config.socket.clone().unwrap();
            if path.is_dir() {
                log::error!("Invalid socket argument");
                return Err(anyhow!("Socket can not be a directory"));
            }
            if !path.is_absolute() {
                log::error!("Invalid socket argument");
                let path = path_to_string(&path)?;
                return Err(anyhow!("Not an absolute path: {path}"));
            }
            path.pop();
            if !path.exists() {
                log::error!("Invalid socket argument");
                let path = path_to_string(&path)?;
                return Err(anyhow!("Invalid path: {path}"));
            }
            if path.is_file() {
                log::error!("Invalid socket argument");
                let path = path_to_string(&path)?;
                return Err(anyhow!("Invalid path: {path}"));
            }
        }
        if config.config.is_none() {
            config.config = get_config(net)?;
        } else {
            let path = config.config.as_ref().unwrap();
            if !path.is_dir() {
                log::error!("Invalid config argument");
                return Err(anyhow!("Config path can not be a directory"));
            }
            if !path.is_absolute() {
                log::error!("Invalid config argument");
                let path = path_to_string(path)?;
                return Err(anyhow!("Not an absolute path: {path}"));
            }
            if !path.exists() {
                log::error!("Invalid config argument");
                let path = path_to_string(path)?;
                return Err(anyhow!("Invalid path does not exist: {path}"));
            }
        }
        log::debug!("The config to run node in {net}: {config:#?}");
        let command = RunCommand::parse_config_to_command(config);
        log::debug!("The command to run a cardano node in {net}: {command}");
        if is_bin_installed("cardano-node").await? {
            let version = check_latest_version("cardano-node").await?;
            let installed = check_installed_version("cardano-node").await?;
            if version.eq(&installed) {
                if proceed("Do you want to download a daily snapshot of the ledger to speed up sync time significantly?")? {
                    download_snapshot(net).await?;
                }
                log::info!("Proceeding to run node in {net}");
                async_command(&command).await?;
            } else {
                log::error!("The installed cardano-node v{installed} is outdated");
                log::error!("Please update to the latest version {version}");
                install_component("cardano-node", false).await?;
                async_command(&command).await?;
            }
        } else {
            log::error!("cardano-node is not installed");
            install_component("cardano-node", false).await?;
            async_command(&command).await?;
        }
        Ok(())
    }

    pub fn parse_config_to_command(config: RunArgs) -> String {
        log::debug!("The parsed config to run node in testnet: {config:#?}");
        let port = config.port;
        let host = config.host;
        let net_config = path_to_string(&config.config.expect("Valid config")).unwrap();
        let db = path_to_string(&config.db.expect("Valid database path")).unwrap();
        let socket = path_to_string(&config.socket.expect("Valid socket path")).unwrap();
        let topology = path_to_string(&config.topology.expect("Valid topology path")).unwrap();
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

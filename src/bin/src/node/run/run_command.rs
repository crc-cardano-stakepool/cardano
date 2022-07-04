use anyhow::{anyhow, Result};
use cardano_multiplatform_lib::NetworkIdKind;
use clap::{Args, Subcommand};
use lib::{
    absolute_ref_path_to_string, async_command, check_installed_version, check_latest_version, install_component, is_bin_installed,
    path_to_string, read_setting, CONFIG_BASE_URL, CONFIG_FILES,
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
        Self::check_config_files(net).await?;
        if config.db.is_none() {
            config.db = Self::get_db(net)?;
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
            config.topology = Self::get_topology(net)?;
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
            config.config = Self::get_config(net)?;
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
    pub fn get_db(network: &str) -> Result<Option<PathBuf>> {
        let key = format!("{network}_db_dir");
        let path = read_setting(&key)?;
        let db = PathBuf::from(&path);
        if !db.exists() {
            log::error!("Invalid db");
            return Err(anyhow!("The path {path} does not exist"));
        }
        if !db.is_dir() {
            log::error!("Invalid db");
            return Err(anyhow!("The path {path} is not a directory"));
        }
        Ok(Some(db))
    }
    pub fn get_topology(network: &str) -> Result<Option<PathBuf>> {
        let key = format!("{network}_config_dir");
        let path = read_setting(&key)?;
        let mut topology = PathBuf::from(&path);
        let key = format!("{network}-topology.json");
        topology.push(key);
        let path = absolute_ref_path_to_string(&topology)?;
        if !topology.exists() {
            log::error!("Invalid topology");
            return Err(anyhow!("The path {path} does not exist"));
        }
        if !topology.is_file() {
            log::error!("Invalid topology");
            return Err(anyhow!("The path {path} is not a file"));
        }
        Ok(Some(topology))
    }
    pub fn get_config(network: &str) -> Result<Option<PathBuf>> {
        let key = format!("{network}_config_dir");
        let path = read_setting(&key)?;
        let mut config = PathBuf::from(&path);
        let key = format!("{network}-config.json");
        config.push(key);
        let path = absolute_ref_path_to_string(&config)?;
        if !config.exists() {
            log::error!("Invalid config");
            return Err(anyhow!("The path {path} does not exist"));
        }
        if !config.is_file() {
            log::error!("Invalid config");
            return Err(anyhow!("The path {path} is not a file"));
        }
        Ok(Some(config))
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

    pub async fn check_config_files(network: &str) -> Result<()> {
        log::debug!("Checking configuration files");
        let key = format!("{network}_config_dir");
        let path = read_setting(&key)?;
        let db = PathBuf::from(&path);
        if !db.exists() {
            return Err(anyhow!("Configuration directory does not exist"));
        }
        for file in CONFIG_FILES {
            Self::check_config_file(db.clone(), network, file).await?;
        }
        Ok(())
    }

    pub async fn check_config_file(mut db: PathBuf, network: &str, file: &str) -> Result<()> {
        let download_path = path_to_string(&db)?;
        let name = format!("{network}-{file}.json");
        db.push(&name);
        let file = path_to_string(&db)?;
        log::debug!("Checking config file {file}");
        if !db.exists() {
            log::warn!("Config file {file} not found, downloading it");
            let cmd = format!("wget {CONFIG_BASE_URL}/{name} -P {download_path}");
            async_command(&cmd).await?;
            log::info!("Downloaded config file {file} successfully");
        }
        log::debug!("Config file found");
        db.pop();
        Ok(())
    }
}

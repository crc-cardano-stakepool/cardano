use crate::{
    match_network, network_to_string, proceed, CardanoComponent, Component,
    Executer, FileSystem, Node, Settings, CONFIG_BASE_URL,
};
use anyhow::{anyhow, Result};
use cardano_multiplatform_lib::NetworkIdKind;
use std::{net::IpAddr, path::PathBuf};

impl Node {
    pub fn get_db(network: NetworkIdKind) -> Result<Option<PathBuf>> {
        let network = &network_to_string(network);
        let key = format!("{network}_db_dir");
        let path = Settings::read(&key)?;
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

    pub fn get_topology(network: NetworkIdKind) -> Result<Option<PathBuf>> {
        let network = &network_to_string(network);
        let key = format!("{network}_config_dir");
        let path = Settings::read(&key)?;
        let mut topology = PathBuf::from(&path);
        let key = format!("{network}-topology.json");
        topology.push(key);
        let path = FileSystem::absolute_ref_path_to_string(&topology)?;
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

    pub fn get_config(network: NetworkIdKind) -> Result<Option<PathBuf>> {
        let network = &network_to_string(network);
        let key = format!("{network}_config_dir");
        let path = Settings::read(&key)?;
        let mut config = PathBuf::from(&path);
        let file_name = format!("{network}-config.json");
        config.push(file_name);
        let path = FileSystem::absolute_ref_path_to_string(&config)?;
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

    pub fn check_config_files(network: NetworkIdKind) -> Result<()> {
        let network = &network_to_string(network);
        log::debug!("Checking configuration files");
        let key = format!("{network}_config_dir");
        let path = Settings::read(&key)?;
        let db = PathBuf::from(&path);
        if !db.exists() {
            return Err(anyhow!("Configuration directory does not exist"));
        }
        let config_files = [
            "config",
            "byron-genesis",
            "shelley-genesis",
            "alonzo-genesis",
            "topology",
        ];
        for file in config_files {
            Self::check_config_file(db.clone(), match_network(network), file)?;
        }
        Ok(())
    }

    pub fn check_config_file(
        mut db: PathBuf,
        network: NetworkIdKind,
        file: &str,
    ) -> Result<()> {
        let network = &network_to_string(network);
        let download_path = FileSystem::path_to_string(&db)?;
        let name = format!("{network}-{file}.json");
        db.push(&name);
        let file = FileSystem::path_to_string(&db)?;
        log::debug!("Checking config file {file}");
        if !db.exists() {
            log::warn!("Config file {file} not found, downloading it");
            let cmd =
                format!("wget {CONFIG_BASE_URL}/{name} -P {download_path}");
            Executer::exec(&cmd)?;
            log::info!("Downloaded config file {file} successfully");
        }
        log::debug!("Config file found");
        db.pop();
        Ok(())
    }

    pub fn handle_db(
        db: Option<PathBuf>,
        network: NetworkIdKind,
    ) -> Result<Option<PathBuf>> {
        if db.is_none() {
            return Self::get_db(network);
        }
        let path = db.as_ref().unwrap();
        if path.is_file() {
            log::error!("Invalid db argument");
            return Err(anyhow!("Database path can not be a file"));
        }
        if !path.is_absolute() {
            log::error!("Invalid db argument");
            let path = FileSystem::path_to_string(path)?;
            return Err(anyhow!("Not an absolute path: {path}"));
        }
        if !path.exists() {
            log::error!("Invalid db argument");
            let path = FileSystem::path_to_string(path)?;
            return Err(anyhow!("Invalid path does not exist: {path}"));
        }
        Ok(db)
    }

    pub fn handle_topology(
        topology: Option<PathBuf>,
        network: NetworkIdKind,
    ) -> Result<Option<PathBuf>> {
        if topology.is_none() {
            return Self::get_topology(network);
        }
        let path = topology.as_ref().unwrap();
        if !path.is_dir() {
            log::error!("Invalid topology argument");
            return Err(anyhow!("Topology path can not be a file"));
        }
        if !path.is_absolute() {
            log::error!("Invalid topology argument");
            let path = FileSystem::path_to_string(path)?;
            return Err(anyhow!("Not an absolute path: {path}"));
        }
        if !path.exists() {
            log::error!("Invalid topology argument");
            let path = FileSystem::path_to_string(path)?;
            return Err(anyhow!("Invalid path does not exist: {path}"));
        }
        Ok(topology)
    }

    pub fn handle_socket(socket: Option<PathBuf>) -> Result<Option<PathBuf>> {
        if socket.is_none() {
            let path = Settings::read("ipc_dir")?;
            let mut socket = PathBuf::from(&path);
            if !socket.exists() {
                log::error!("Invalid socket");
                return Err(anyhow!("The path {path} does not exist"));
            }
            socket.push("node.socket");
            return Ok(Some(socket));
        }
        let mut path = socket.clone().unwrap();
        if path.is_dir() {
            log::error!("Invalid socket argument");
            return Err(anyhow!("Socket can not be a directory"));
        }
        if !path.is_absolute() {
            log::error!("Invalid socket argument");
            let path = FileSystem::path_to_string(&path)?;
            return Err(anyhow!("Not an absolute path: {path}"));
        }
        path.pop();
        if !path.exists() {
            log::error!("Invalid socket argument");
            let path = FileSystem::path_to_string(&path)?;
            return Err(anyhow!("Invalid path: {path}"));
        }
        if path.is_file() {
            log::error!("Invalid socket argument");
            let path = FileSystem::path_to_string(&path)?;
            return Err(anyhow!("Invalid path: {path}"));
        }
        Ok(socket)
    }

    pub fn handle_config(
        config: Option<PathBuf>,
        network: NetworkIdKind,
    ) -> Result<Option<PathBuf>> {
        if config.is_none() {
            return Self::get_config(network);
        }
        let path = config.as_ref().unwrap();
        if !path.is_dir() {
            log::error!("Invalid config argument");
            return Err(anyhow!("Config path can not be a directory"));
        }
        if !path.is_absolute() {
            log::error!("Invalid config argument");
            let path = FileSystem::path_to_string(path)?;
            return Err(anyhow!("Not an absolute path: {path}"));
        }
        if !path.exists() {
            log::error!("Invalid config argument");
            let path = FileSystem::path_to_string(path)?;
            return Err(anyhow!("Invalid path does not exist: {path}"));
        }
        Ok(config)
    }

    pub fn run(
        cmd: &str,
        network: NetworkIdKind,
        db: Option<PathBuf>,
    ) -> Result<()> {
        let network = &network_to_string(network);
        if CardanoComponent::is_component_installed(Component::Node)? {
            let version =
                CardanoComponent::check_latest_version(Component::Node)?;
            let installed =
                CardanoComponent::check_installed_version(Component::Node)?;
            if version.eq(&installed) {
                if db.as_ref().unwrap().read_dir()?.next().is_none()
                    && proceed(
                        "Do you want to download a daily snapshot of \
                        the ledger to speed up sync time significantly?",
                    )?
                {
                    Self::download_snapshot(match_network(network))?;
                }
                log::info!("Proceeding to run node in {network}");
                Executer::exec(cmd)?;
                return Ok(());
            }
            log::error!("The installed cardano-node v{installed} is outdated");
            log::error!("Please update to the latest version {version}");
            CardanoComponent::install_component(Component::Node)?;
            Executer::exec(cmd)?;
            return Ok(());
        }
        CardanoComponent::install_component(Component::Node)?;
        Executer::exec(cmd)?;
        Ok(())
    }

    pub fn parse_config(
        port: u16,
        host: IpAddr,
        config: Option<PathBuf>,
        db: &Option<PathBuf>,
        socket: Option<PathBuf>,
        topology: Option<PathBuf>,
    ) -> String {
        let net_config =
            FileSystem::path_to_string(config.as_ref().expect("Valid config"))
                .unwrap();
        let db = FileSystem::path_to_string(
            db.as_ref().expect("Valid database path"),
        )
        .unwrap();
        let socket = FileSystem::path_to_string(
            socket.as_ref().expect("Valid socket path"),
        )
        .unwrap();
        let topology = FileSystem::path_to_string(
            topology.as_ref().expect("Valid topology path"),
        )
        .unwrap();
        let cmd = format!(
            "cardano-node run \
            --topology {topology} \
            --database-path {db} \
            --socket-path {socket} \
            --host-addr {host} \
            --port {port} \
            --config {net_config} \
            "
        );
        log::debug!("The command to run a cardano node: {cmd}");
        cmd
    }
}

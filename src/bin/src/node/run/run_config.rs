use clap::Args;
use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;

#[derive(Debug, Args)]
pub struct RunConfig {
    #[clap(long, value_parser = clap::value_parser!(u16).range(1..), default_value_t = 3001)]
    pub port: u16,
    #[clap(long, parse(from_os_str), default_value = "$DATA_DIR/$NETWORK")]
    pub db: PathBuf,
    #[clap(long, value_parser, default_value_t = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))]
    pub host: IpAddr,
    #[clap(long, parse(from_os_str), default_value = "$CONFIG_DIR/$NETWORK/$NETWORK-topology.json")]
    pub topology: PathBuf,
    #[clap(long, parse(from_os_str), default_value = "$IPC_DIR/node.socket")]
    pub socket: PathBuf,
    #[clap(long, parse(from_os_str), default_value = "$CONFIG_DIR/$NETWORK/$NETWORK-config.json")]
    pub config: PathBuf,
}

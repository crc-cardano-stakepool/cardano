use clap::Args;
use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Debug, Args)]
pub struct RunConfig {
    #[clap(long, value_parser, default_value_t = 3001)]
    port: u16,
    #[clap(long, value_parser, default_value = "$DATA_DIR/$NETWORK")]
    db: PathBuf,
    #[clap(long, value_parser, default_value_t = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))]
    host: IpAddr,
    #[clap(long, value_parser, default_value = "$CONFIG_DIR/$NETWORK/$NETWORK-topology.json")]
    topology: PathBuf,
    #[clap(long, value_parser, default_value = "$IPC_DIR/node.socket")]
    socket: PathBuf,
    #[clap(long, value_parser, default_value = "$CONFIG_DIR/$NETWORK/$NETWORK-config.json")]
    config: PathBuf,
}

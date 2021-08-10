
use structopt::StructOpt;
use std::path::PathBuf;
use std::net::IpAddr;

#[derive(Debug, StructOpt)]
pub struct RunConfig {
    #[structopt(short, long, default_value = "3001")]
    port: u16,
    #[structopt(short, long, parse(from_os_str))]
    db: PathBuf,
    #[structopt(short, long, default_value = "127.0.0.1")]
    host: IpAddr,  
    #[structopt(short, long, parse(from_os_str))]
    topology: PathBuf,
    #[structopt(short, long, parse(from_os_str))]
    socket: PathBuf,
    #[structopt(short, long, parse(from_os_str))]
    config: PathBuf
}
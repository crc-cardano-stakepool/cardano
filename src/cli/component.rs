use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum CardanoComponent {
    #[structopt(about = "Install cardano-node")]
    Node,
    #[structopt(about = "Install cardano-cli")]
    Cli,
    #[structopt(about = "Install cardano-wallet")]
    Wallet,
    #[structopt(about = "Install cardano-db-sync")]
    Db,
}

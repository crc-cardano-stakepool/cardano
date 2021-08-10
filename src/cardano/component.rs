use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum CardanoComponent {
    Node,
    Cli,
    Wallet,
    Db,
}

pub mod cardano {
    use structopt::StructOpt;

    #[derive(Debug, StructOpt)]
    pub enum Command {
        Node {},
        Cli {},
        Wallet {},
        Tx {},
        Mint {},
        Address {},
        Db {},
        Graphql {},
        Ledger {},
        Rosetta {},
        Plutus {},
        Marlowe {},
        Explorer {},
        Smash {},
        Install {},
        Update {},
        Config {},
        Uninstall {},
    }

    pub fn start(cmd: Command) {
        match cmd {
            Command::Node{}=> println!("Called node"),
            Command::Cli{} => println!("Called cli"),
            Command::Wallet{} => println!("Called wallet"),
            Command::Tx{} => println!("Called tx"),
            Command::Mint{} => println!("Called mint"),
            Command::Address{} => println!("Called address"),
            Command::Db{} => println!("Called db"),
            Command::Graphql{} => println!("Called graphql"),
            Command::Ledger{} => println!("Called ledger"),
            Command::Rosetta{} => println!("Called rosetta"),
            Command::Plutus{} => println!("Called plutus"),
            Command::Marlowe{} => println!("Called marlowe"),
            Command::Explorer{} => println!("Called explorer"),
            Command::Smash{} => println!("Called smash"),
            Command::Install{} => println!("Called install"),
            Command::Update{} => println!("Called install"),
            Command::Config{} => println!("Called config"),
            Command::Uninstall{} => println!("Called uninstall"),
        }
    }
}

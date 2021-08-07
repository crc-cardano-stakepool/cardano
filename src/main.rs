use structopt::StructOpt;
mod cardano;
use crate::cardano::cardano::*;

fn main() {
    Cardano::start(Cardano::from_args().cmd);
}

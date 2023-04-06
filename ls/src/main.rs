mod lib;
use lib::Config;
use clap::Parser;

fn main() {
    let config = Config::parse();

    lib::run(&config).unwrap();
}

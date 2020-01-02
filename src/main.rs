extern crate reqwest;

use std::env;
use std::process::exit;

mod cli;
mod github_client;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Err(e) = cli::run(args) {
        println!("Error! : {}", e);

        exit(1)
    }
}

use std::env;
use std::process::exit;

mod cli;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Err(e) = cli::run(args) {
        println!("Error! : {}", e);

        exit(1)
    }
}

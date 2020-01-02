use std::process::exit;

mod cli;

fn main() {
    if let Err(e) = cli::run() {
        println!("Error! : {}", e);

        exit(1)
    }
}

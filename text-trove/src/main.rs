use std::env;
use std::process;
use text_trove::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = text_trove::run(config) {
        eprintln!("application error: {e}");
        process::exit(1);
    }
}
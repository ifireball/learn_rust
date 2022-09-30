use std::env;
use std::process;

use learn_rust::minigrep::Config;
use learn_rust::minigrep;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(
        |err| {
            eprintln!("Can't read config: {err}");
            process::exit(1);
        }
    );

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

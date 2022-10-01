use std::env;
use std::process;

use learn_rust::minigrep::Config;
use learn_rust::minigrep;

fn main() {
    let config = Config::build(env::args())
        .unwrap_or_else(|err| {
            eprintln!("Can't read config: {err}");
            process::exit(1);
        });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

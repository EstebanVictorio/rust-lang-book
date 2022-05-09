// This is a challenge lesson that will guide us through a variety of concepts that we already
// have to build a program that works as a simple version of the "grep" command.

use cli_program::cli::args::Search;
use std::env;
use std::process;

fn main() {
    let search = Search::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    if let Err(err) = search.run() {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}

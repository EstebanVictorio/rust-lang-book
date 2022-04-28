// This is a challenge lesson that will guide us through a variety of concepts that we already
// have to build a program that works as a simple version of the "grep" command.

use cli_program::cli::args::{get_args, Search};
use std::process;

fn main() {
    let args = get_args();
    let search = Search::new(&args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    if let Err(err) = search.run() {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}

use gor::cli::cli::CLI;
use std::env;

fn main() {
    let cli = CLI::new(env::args().collect());
    cli.execute();
}

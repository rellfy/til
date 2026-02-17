use clap::Parser;
use cli::Cli;

mod cli;

fn main() {
    if let Err(e) = Cli::parse().run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

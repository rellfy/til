mod cli;

fn main() {
    let input = std::env::args().collect::<String>();
    cli::process(&input);
}

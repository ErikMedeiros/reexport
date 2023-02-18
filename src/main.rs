use clap::Parser;
use reexport::{read_dirs, CLI};

fn main() {
    let cli = CLI::parse();

    let contents = read_dirs(&cli.paths, &cli.ignores);
    println!("{:?}", contents);
}

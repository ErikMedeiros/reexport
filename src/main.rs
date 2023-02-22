use clap::Parser;
use reexport::{read_path, CLI};

fn main() {
    let cli = CLI::parse();

    let hash = read_path(&cli.paths, cli.depth);

    for (key, values) in hash {
        println!("{:?}", key);

        for value in values {
            println!("  {:?}", value);
        }
    }
}

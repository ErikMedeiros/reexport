use clap::Parser;
use reexport::{read_path, Entry, CLI};

fn main() {
    let cli = CLI::parse();
    for path in &cli.paths {
        let output = read_path(&path, &cli.extensions, &cli.ignore, cli.depth, 0);
        //println!("{:?}", output);

        for entry in output {
            print_entry(&entry, 0);
        }
    }
}

fn print_entry(entry: &Entry, depth: u32) {
    for _ in 0..depth {
        print!("    ");
    }

    match entry {
        Entry::File(file) => println!("file -> {:?}", file.file_name().unwrap()),
        Entry::Folder { entries, name } => {
            println!("folder -> {:?}", name);
            for entry in entries {
                print_entry(&entry, depth + 1);
            }
        }
    }
}

use clap::Parser;
use reexport::{read_path, write_files, Entry, CLI};

fn main() {
    let cli = CLI::parse();
    for path in &cli.paths {
        let entries = read_path(&path, &cli.ignore, cli.recursive, cli.only_ts, cli.depth, 0);
        write_files(path, &entries);

        for entry in entries {
            print_entry(&entry, 0);
        }
    }
}

fn print_entry(entry: &Entry, depth: u32) {
    for _ in 0..depth {
        print!("    ");
    }

    match entry {
        Entry::File(file) => println!("file -> {:?}", file.file_name().unwrap_or_default()),
        Entry::Folder { entries, path } => {
            println!("folder -> {:?}", path.file_name().unwrap_or_default());
            for entry in entries {
                print_entry(&entry, depth + 1);
            }
        }
    }
}

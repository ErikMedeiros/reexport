use std::path::{Path, PathBuf};
use std::{ffi::OsString, fs};

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
pub struct CLI {
    /// List of paths to be reexported
    #[arg(required = true)]
    pub paths: Vec<PathBuf>,

    /// Exclude files with matching names
    #[arg(short, long, default_values = ["index"])]
    pub ignore: Vec<OsString>,

    /// File extensions to consider
    #[arg(long = "ext", default_values = [".ts", ".tsx", ".js", ".jsx"])]
    pub extensions: Vec<OsString>,

    /// Reexport folders until specific depth
    #[arg(short, long, default_value_t = 1)]
    pub depth: usize,
}

pub fn read_path(root: &Path, max_depth: usize, depth: usize) -> Vec<Entry> {
    let rd = fs::read_dir(root).unwrap();

    let output: Vec<_> = rd
        .filter_map(|r| r.ok())
        .filter_map(|entry| {
            let path = entry.path();

            if path.is_file() {
                return Some(Entry::File(path));
            } else if depth + 1 <= max_depth {
                let folder = Entry::Folder {
                    name: path.file_name().unwrap().to_owned(),
                    entries: read_path(&path, max_depth, depth + 1),
                };
                return Some(folder);
            }

            return None;
        })
        .collect();

    return output;
}

#[derive(Debug)]
pub enum Entry {
    Folder { name: OsString, entries: Vec<Entry> },
    File(PathBuf),
}

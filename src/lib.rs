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

    /// Reexport subfolders within N depth
    #[arg(short, long, default_value_t = 0)]
    pub depth: usize,
}

pub fn read_path(root: &Path, max_depth: usize, depth: usize) -> Vec<Entry> {
    let rd = fs::read_dir(root).unwrap();

    let output = rd
        .filter_map(|r| r.ok())
        .map(|entry| {
            let path = entry.path();

            if path.is_file() {
                return Entry::File(path);
            }

            let name = path.file_name().unwrap().to_owned();
            let entries = if depth + 1 <= max_depth {
                read_path(&path, max_depth, depth + 1)
            } else {
                Vec::new()
            };

            return Entry::Folder { name, entries };
        })
        .collect::<Vec<Entry>>();

    return output;
}

#[derive(Debug)]
pub enum Entry {
    Folder { name: OsString, entries: Vec<Entry> },
    File(PathBuf),
}

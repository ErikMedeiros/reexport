use std::{collections::HashMap, ffi::OsString, path::PathBuf};

use walkdir::WalkDir;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
pub struct CLI {
    /// List of paths to be reexported
    #[arg(required = true)]
    pub paths: Vec<OsString>,

    /// Exclude files with matching names
    #[arg(short, long, default_values = ["index"])]
    pub ignore: Vec<String>,

    /// File extensions to consider
    #[arg(long = "ext", default_values = [".ts", ".tsx", ".js", ".jsx"])]
    pub extensions: Vec<String>,

    /// Reexport folders until specific depth
    #[arg(short, long, default_value_t = 1)]
    pub depth: usize,
}

pub fn read_path(paths: &Vec<OsString>, depth: usize) -> HashMap<&OsString, Vec<PathBuf>> {
    let mut hash = HashMap::new();

    for path in paths {
        let files: Vec<_> = WalkDir::new(path)
            .max_depth(depth)
            .into_iter()
            .filter_map(|r| r.ok())
            .filter(|entry| !entry.metadata().unwrap().is_dir())
            .map(|entry| entry.path().to_owned())
            .collect();

        hash.insert(path, files);
    }

    return hash;
}

/*
paths
    .into_iter()
    .map(|path| fs::read_dir(path))
    .filter_map(|r| r.and_then(|rd| Ok(rd)).ok())
    .flat_map(|rd| rd.map(|entry| entry))
    .collect();
 */

use std::{ffi::OsString, fs, slice::Iter};

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
pub struct CLI {
    /// List of relative paths to be reexported
    #[arg(required = true)]
    pub paths: Vec<OsString>,

    /// Exclude files with matching names
    #[arg(short, long, default_values = ["index"])]
    pub ignore: Vec<String>,

    /// File extensions to consider
    #[arg(long = "ext", default_values = [".ts", ".tsx", ".js", ".jsx"])]
    pub extensions: Vec<String>,
}

pub fn read_dirs(
    paths: &Vec<OsString>,
    ignore: &Vec<String>,
    extensions: &Vec<String>,
) -> Vec<Vec<fs::DirEntry>> {
    paths
        .into_iter()
        .filter_map(|path| fs::read_dir(path).ok())
        .map(|rd| {
            rd.filter_map(|r| r.ok())
                .filter_map(|entry| filter_dir(entry, ignore, extensions))
                .collect()
        })
        .collect()
}

fn filter_dir(
    entry: fs::DirEntry,
    ignore: &Vec<String>,
    include: &Vec<String>,
) -> Option<fs::DirEntry> {
    let name = entry.file_name().into_string().ok()?;

    let include_name_substr = |mut iter: Iter<String>| iter.any(|str| name.contains(str));
    let should_exclude = include_name_substr(ignore.into_iter());
    let should_include = include_name_substr(include.into_iter());

    if !should_exclude && should_include {
        return Some(entry);
    }
    return None;
}

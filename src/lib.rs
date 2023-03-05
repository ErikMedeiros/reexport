use std::path::{Path, PathBuf};
use std::{ffi::OsString, fs};

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
#[command(group(clap::ArgGroup::new("traversal").args(["recursive", "depth"])))]
pub struct CLI {
    /// List of paths to be reexported
    #[arg(required = true)]
    pub paths: Vec<PathBuf>,

    /// Ignore files with matching names
    #[arg(short, long)]
    pub ignore: Vec<OsString>,

    /// Ignore .js or .jsx files
    #[arg(long)]
    pub only_ts: bool,

    /// Reexport subfolders within N depth
    #[arg(short, long, default_value_t = 0)]
    pub depth: usize,

    /// Reexport all subfolders recursively
    #[arg(short, long)]
    pub recursive: bool,
}

pub enum Entry {
    Folder { path: PathBuf, entries: Vec<Entry> },
    File(PathBuf),
}

pub fn write_files(root: &Path, entries: &Vec<Entry>) {
    let mut buffer = Vec::new();

    for entry in entries {
        match entry {
            Entry::File(path) => {
                let os_str = path.file_stem().unwrap_or_default();
                let name = os_str.to_str().unwrap_or_default();
                buffer.push(format!("export * from './{}';", name));
            }
            Entry::Folder { path, entries } => {
                let os_str = path.file_stem().unwrap_or_default();
                let name = os_str.to_str().unwrap_or_default();
                buffer.push(format!("export * from './{}';", name));
                write_files(&path, entries);
            }
        }
    }

    if buffer.len() > 0 {
        let mut output_path = root.to_owned();
        output_path.push("index.ts");
        fs::write(output_path, buffer.join("\n") + "\n").unwrap();
    }
}

pub fn read_path(
    root: &Path,
    ignore: &Vec<OsString>,
    only_ts: bool,
    recursive: bool,
    max_depth: usize,
    depth: usize,
) -> Vec<Entry> {
    let rd = fs::read_dir(root).unwrap();

    let output = rd
        .filter_map(|r| r.ok())
        .filter(|dir_entry| filter_dir_entry(dir_entry, only_ts, ignore))
        .map(|entry| {
            let path = entry.path();

            if path.is_file() {
                return Entry::File(path);
            }

            let entries = if recursive || depth + 1 <= max_depth {
                read_path(&path, ignore, only_ts, recursive, max_depth, depth + 1)
            } else {
                Vec::new()
            };

            return Entry::Folder { path, entries };
        })
        .collect::<Vec<Entry>>();

    return output;
}

fn filter_dir_entry(entry: &fs::DirEntry, only_ts: bool, ignore: &Vec<OsString>) -> bool {
    let path = entry.path();
    let name = path
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();

    let should_exclude = name.contains("index")
        || ignore
            .iter()
            .any(|t| name.contains(t.to_str().unwrap_or_default()));

    if path.is_file() {
        let should_include = if let Some(extension) = path.extension() {
            let allowed_extensions = if only_ts {
                vec!["ts", "tsx"]
            } else {
                vec!["ts", "tsx", "js", "jsx"]
            };

            let extension = extension.to_str().unwrap_or_default();
            allowed_extensions.iter().any(|ext| extension == *ext)
        } else {
            false
        };
        return !should_exclude && should_include;
    }
    return !should_exclude;
}

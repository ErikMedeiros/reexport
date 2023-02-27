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
    extensions: &Vec<OsString>,
    ignore: &Vec<OsString>,
    max_depth: usize,
    depth: usize,
) -> Vec<Entry> {
    let rd = fs::read_dir(root).unwrap();

    let output = rd
        .filter_map(|r| r.ok())
        .filter(|dir_entry| filter_dir_entry(dir_entry, extensions, ignore))
        .map(|entry| {
            let path = entry.path();

            if path.is_file() {
                return Entry::File(path);
            }

            let entries = if depth + 1 <= max_depth {
                read_path(&path, extensions, ignore, max_depth, depth + 1)
            } else {
                Vec::new()
            };

            return Entry::Folder { path, entries };
        })
        .collect::<Vec<Entry>>();

    return output;
}

fn filter_dir_entry(
    entry: &fs::DirEntry,
    extensions: &Vec<OsString>,
    ignore: &Vec<OsString>,
) -> bool {
    let path = entry.path();
    let name = path.file_name().unwrap_or_default();

    let should_exclude = ignore.iter().any(|t| {
        name.to_str()
            .unwrap_or_default()
            .contains(t.to_str().unwrap_or_default())
    });

    if path.is_file() {
        let should_include = extensions.iter().any(|ext| {
            name.to_str()
                .unwrap_or_default()
                .ends_with(ext.to_str().unwrap_or_default())
        });
        return !should_exclude && should_include;
    }
    return !should_exclude;
}

pub enum Entry {
    Folder { path: PathBuf, entries: Vec<Entry> },
    File(PathBuf),
}

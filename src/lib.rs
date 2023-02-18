use std::{ffi::OsString, fs};

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
pub struct CLI {
    /// List of relative paths to be reexported
    #[arg(required = true)]
    pub paths: Vec<OsString>,

    /// Ignore files with thoses texts in it
    #[arg(short, long, default_values = ["index"])]
    pub ignores: Vec<String>,
}

pub fn read_dirs(paths: &Vec<OsString>, ignores: &Vec<String>) -> Vec<Vec<String>> {
    paths
        .into_iter()
        .map(|p| fs::read_dir(p))
        .filter_map(|r| r.ok())
        .map(|rd| {
            rd.filter_map(|r| r.ok())
                .filter(|dr| {
                    !ignores
                        .into_iter()
                        .any(|i| dr.file_name().into_string().unwrap_or_default().contains(i))
                })
                .map(|dr| dr.file_name().into_string())
                .filter_map(|r| r.ok())
                .collect()
        })
        .collect()
}

mod refresh;
use crate::config;

use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub fn refresh(cfg: config::Config) {
    // Learn what Agnosticd directories exist
    let potential_dirs = find_provisioned_agnosticd_dirs(cfg);
    // Learn which clusters are up
    // Update our cached info
}

fn find_provisioned_agnosticd_dirs(cfg: config::Config) -> Vec<PathBuf> {
    let mut found_dirs: Vec<PathBuf> = Vec::new();

    for r in cfg.agnosticd_resource_dirs {
        println!("Looking at: {}", r);
        match read_dirs_from_dir(&r) {
            Ok(v) => {
                found_dirs.extend(v);
            }
            Err(e) => {
                println!("Error {} reading {}", e, r);
            }
        }
    }
    println!("found_dirs: {:?}", found_dirs);
    return found_dirs;
}

// Example from: https://stackoverflow.com/questions/37439327/how-to-write-a-function-that-returns-vecpath
pub fn _read_filenames_from_dir<P>(path: P) -> Result<Vec<PathBuf>, io::Error>
where
    P: AsRef<Path>,
{
    fs::read_dir(path)?
        .into_iter()
        .map(|x| x.map(|entry| entry.path()))
        .collect()
}

pub fn read_dirs_from_dir<P>(path: P) -> Result<Vec<PathBuf>, io::Error>
where
    P: AsRef<Path>,
{
    fs::read_dir(path)?
        .into_iter()
        .map(|x| x.map(|entry| entry.path()))
        .filter(|x| x.as_ref().is_ok_and(|f| f.is_dir()))
        .collect()
}

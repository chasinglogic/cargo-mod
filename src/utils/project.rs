use std::path::{Path, PathBuf};
use std::env;
use std::fs;

pub enum Where {
    ProjectRoot,
    Other,
}

pub enum ModFile {
    Library,
    Binary,
    Mod,
    Both,
}

pub fn kind_of_crate(target_path: &PathBuf) -> CrateType {
    let mut lib_path = target_path.clone();
    lib_path.push("lib.rs");

    let mut bin_path = target_path.clone();
    bin_path.push("main.rs");

    if fs::metadata(lib_path.as_path()).is_ok() && 
       fs::metadata(bin_path.as_path()).is_ok() {
        return ModFile::Both
    }

    if fs::metadata(lib_path.as_path()).is_ok() {
        return ModFile::Library
    }

    if fs::metadata(bin_path.as_path()).is_ok() {
        return ModFile::Binary
    }

    ModFile::Mod
}

fn gen_path_recurse(cur_dir: &Path) -> PathBuf {
    match where_are_we(cur_dir) {
        Where::ProjectRoot => cur_dir.to_path_buf(),
        Where::Other => {
            if cur_dir.parent() == None {
                panic!("Not a cargo project, please run again in a Cargo project.");
            }

            gen_path_recurse(&cur_dir.parent().unwrap())
        },
    }
}

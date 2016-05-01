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

pub fn find_project_root() -> PathBuf {
    // Leaving this unwrap, we want to fail if for some reason we can't find current directory
    let generated_path = gen_path_recurse(env::current_dir().unwrap().as_path());
    println!("Generated: {}", generated_path.display());
    generated_path
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

fn where_are_we(cur_dir: &Path) -> Where {
    // If Cargo.toml is here then we are at project root
    let mut check = cur_dir.to_path_buf().clone();
    check.push("Cargo.toml");

    if fs::metadata(check).is_ok() {
        return Where::ProjectRoot
    }

    Where::Other
}





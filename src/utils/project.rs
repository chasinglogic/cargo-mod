use std::path::{Path, PathBuf};
use std::env;
use std::fs;



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

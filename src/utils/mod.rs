pub mod project;
pub mod module;

use std::path::PathBuf;

pub fn pretty_path(root: &PathBuf, target: &PathBuf) -> PathBuf {
    target.strip_prefix(root).unwrap().to_path_buf()
}

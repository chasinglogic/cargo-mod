use std::path::{Path, PathBuf};
use std::env;
use std::fs;

pub enum Where {
    ProjectRoot,
    Other,
}

pub fn find_project_root(name: &String) -> PathBuf {
    let generated_path = gen_path_recurse(env::current_dir().unwrap().as_path(), name);
    println!("Generated: {}", generated_path);
    let p = Path::new(&generated_path);
    p.to_path_buf()
}

fn gen_path_recurse(cur_dir: &Path, target: &String) -> String {
    // #crossplatformishard
    let slash = if cfg!(target_os = "windows") {
        "\\"
    } else {
        "/"
    };

    println!("Current directory: {}", cur_dir.display());

    match where_are_we(cur_dir) {
        Where::ProjectRoot => format!("{0}{1}src{1}{2}{1}", cur_dir.to_str().unwrap(), slash, target),
        Where::Other => {
            if cur_dir.parent() == None {
                panic!("Not a cargo project, please run again in a Cargo project.");
            }

            gen_path_recurse(&cur_dir.parent().unwrap(), target)
        },
    }
}

fn where_are_we(cur_dir: &Path) -> Where {
    // If Cargo.toml is here then we are at project root
    if fs::metadata(Path::new("Cargo.toml")).is_ok() {
        return Where::ProjectRoot
    }

    Where::Other
}

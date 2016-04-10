extern crate getopts;

use getopts::Options;

use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::fs::File;
use std::fs;
use std::env;

enum Where {
    ProjectRoot,
    SrcDirRoot,
}

fn print_usage() {
    println!("Work in progress.")
}

fn where_are_we() -> Where {
    Where::ProjectRoot
}

fn create_directory_or_panic(p: &Path) {
    match fs::create_dir(p) {
        Err(e) => panic!("Unable to create directory: {}", e),
        Ok(_) => println!("Created directory: {}", p.to_str().unwrap()),
    }
}

fn open_file_or_panic(p: &Path) -> File {
    match File::create(p) {
        Err(e) => panic!("Unable to open file: {}", e),
        Ok(file) => file,
    }
}

fn write_file_or_panic(mut f: &File, b: &[u8]) {
    match f.write_all(b) {
        Err(e) => panic!("Unable to write to file: {}", e),
        Ok(_) => println!("Generated module files."),
    }
}

fn gen_path(name: String) -> PathBuf {
    // #crossplatformishard
    let slash = if cfg!(target_os = "windows") {
        "\\"
    } else {
        "/"
    };

    // Get the current directory
    let cur_dir = env::current_dir().unwrap();

    let first_path = match where_are_we() {
        Where::ProjectRoot => format!("src/{}{}mod.rs", name, slash),
        Where::SrcDirRoot => format!("{}{}mod.rs", name, slash),
    }

    let p = Path::new(&first_path);
        return p.to_path_buf()
    }

    p.to_path_buf()
}

fn gen_folder_module(name: String, private: bool) {
    let path = gen_path(name).as_path();
    // create_directory_or_panic(&name);
    // let file = open_file_or_panic(&path_string);
    // write_file_or_panic(&file, format!("pub mod {}", name).as_bytes());
}

// fn gen_module(name: String, private: bool) {
//     let path_string = format!("{}.rs", name);
//     let mut file = open_file_or_panic(&path_string);
// }

fn main() {
    let mut opts = Options::new();
    let args: Vec<String> = env::args().collect();

    opts.optflag("p", "private", "Make the generated module private.");
    opts.optflag("f", "folder", "Generate a folder module instead of a file.");

    let matches = match opts.parse(&args) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    let private = matches.opt_present("p");
    let folder = matches.opt_present("f");
    let name = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage();
        return
    };

    // if folder {
        gen_folder_module(name, private);
        // return 
    // }

    // gen_module(name, private)
}

mod utils;

extern crate getopts;

use getopts::Options;

use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::fs;
use std::env;

fn print_usage() {
    println!("Work in progress.")
}

fn pretty_print_path(root: &PathBuf, target: &PathBuf) -> PathBuf {
    target.strip_prefix(root.parent().unwrap().parent().unwrap()).unwrap().to_path_buf()
}

fn gen_folder_module(name: String, private: bool) {
    let root_path = utils::pr::find_project_root(&name);
    let mut our_path = root_path.clone();

    let res = fs::create_dir(our_path.as_path());
    if res.is_err() {
        panic!("Unable to create directory: {}", res.err().unwrap());
    } 
    println!("Created directory: {}", 
             pretty_print_path(&root_path, &our_path).display());

    our_path.push("mod.rs");
    let mut f = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(our_path.as_path())
            .unwrap();

    let mod_line = format!("pub mod {}", &name);
    let result = f.write_all(mod_line.as_bytes());
    if result.is_err() {
        panic!("Unable to write to file: {}", result.err().unwrap());
    }
    println!("Generated mod file: {}", 
             pretty_print_path(&root_path, &our_path).display()); 
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
        matches.free[1].clone()
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

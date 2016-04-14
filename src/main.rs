mod utils;

extern crate getopts;

use getopts::Options;

use utils::*;

use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::io;
use std::fs;
use std::env;

fn print_usage() {
    println!("Work in progress.")
}

fn gen_folder_module(name: String, private: bool) {
    let root_path = project::find_project_root(); // PathBuf
    let mut our_path = root_path.clone();
    our_path.push("src");
    our_path.push(&name);

    let res = fs::create_dir(our_path.as_path());
    if res.is_err() {
        println!("Unable to create directory: {}", res.err().unwrap());
        std::process::exit(1);
    } 
    println!("Created directory: {}", 
             path::pretty_path(&root_path, &our_path).display());

    our_path.push("mod.rs");
    let mut f = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(our_path.as_path())
            .unwrap();

    let result = f.write_all("".as_bytes());
    if result.is_err() {
        panic!("Unable to write to file: {}", result.err().unwrap());
    }
    println!("Generated mod file: {}", 
             path::pretty_path(&root_path, &our_path).display()); 

    module::add_mod(&root_path, module::generate_modstring(name, private)) 
}

fn gen_module(name: String, private: bool) {
    let root_path = project::find_project_root();
    let our_path = root.clone();
    our_path.push("src");

    // Then we have a folder module which we want to add to.
    // if name.contains("/") {
    //     let iter = name.split("/");
    // }
    
    let mut namers = name.clone();
    namers.push(".rs");
    our_path.push(namers);

    let mut f = fs::File::create(our_path.as_path());
    f.write_all("");
    println!("Created empty file: ", path::pretty_path(&root_path, &our_path));

    module::add_mod(&root_path, module::generate_modstring(name, private))
}

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

    if folder {
        gen_folder_module(name, private);
        return 
    }

    gen_module(name, private)
}

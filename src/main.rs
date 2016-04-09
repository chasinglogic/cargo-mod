extern crate getopts;

use getopts::Options;

use std::path::Path;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::env;

fn print_usage() {
    println!("Work in progress.")
}

fn open_file_or_panic(p: &str) -> File {
    let path = Path::new(p);
    match File::create(&path) {
        Err(e) => panic!("Unable to open file: {}", e),
        Ok(file) => file,
    }
}

fn write_file_or_panic(f: &File, b: Bytes) {
    match f.write_all(b) {
        Err(e) => panic!("Unable to write to file: {}", e),
        Ok(_) => println!("Generated module files."),
    }
}

fn gen_folder_module(name: String, private: bool) {
    let path_string = if cfg!(target_os = "windows") {
        format!("{}\\mod.rs", name)
    } else {
        format!("{}/mod.rs", name)
    };

    let mut file = open_file_or_panic(&path_string);
    write_file_or_panic(&file, format!("pub mod {}", name).as_bytes())
}

fn gen_module(name: String, private: bool) {
    let path_string = format!("{}.rs", name);
    let mut file = open_file_or_panic(&path_string);

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
        matches.free[0].clone()
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

pub mod utils;
mod module;

extern crate getopts;

use getopts::Options;
use std::{env, process};

fn print_usage() {
    println!("Work in progress.")
}

fn main() {
    let mut opts = Options::new();
    let args: Vec<String> = env::args().collect();

    opts.optflag("p", "private", "Make the generated module private.");
    opts.optflag("h", "help", "Show help message");

    let matches = match opts.parse(&args) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage();
        process::exit(0);
    }

    let private = matches.opt_present("p");
    let name = if !matches.free.is_empty() {
        matches.free[1].clone()
    } else {
        print_usage();
        process::exit(1);
    };

    if !utils::are_in_project() {
        println!("Please run this command inside a Cargo project. Exiting.");
        process::exit(1);
    }

    let mut current_dir = env::current_dir()
        .expect("Unexpected Error: Cannot get current working directory.");
    module::gen_module(name, private, &mut current_dir)
}

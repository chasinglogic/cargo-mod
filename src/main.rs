mod module;

extern crate getopts;

use getopts::Options;
use std::env;

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
        return print_usage()
    }

    let private = matches.opt_present("p");
    let name = if !matches.free.is_empty() {
        matches.free[1].clone()
    } else {
        return print_usage();
    };

    module::gen_module(name, private)
}

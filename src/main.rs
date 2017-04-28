pub mod utils;
mod module;

extern crate getopts;

use getopts::Options;
use std::{env, process};

fn print_usage() {
    println!("
Create a new module or modules in the current cargo project.

Usage:
  cargo mod [<options>] [<path>]
  cargo mod -h | --help

Options:
  -h, --help        Print this message
  -p, --private     Make the generated module/s private

Details: The path is a path seperated by / (even if on windows for
now.) and will generate all folder modules in between the final
module and beginning module. The starting point being the current
working directory.

Example:
If you are in the root of your project and you run

cargo mod this/is/a/module

We will generate 3 folder modules

this
is
a

and 1 file module

module.rs

With a final directory structure of:

my_crate/
 - Cargo.toml
 - src/
    - lib.rs
    - this/
      - mod.rs
      - is/
        - mod.rs
        - a/
          - mod.rs
          - module.rs

If you want to only generate one module you can denote whether it is a
folder or file module by the addition or omission of a trailing /

Example folder:
cargo mod new/

Example file:
cargo mod new
")

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
    let mut name = if !matches.free.is_empty() {
        matches.free[1].clone()
    } else {
        print_usage();
        process::exit(1)
    };

    // Hack to get around how cargo subcommands and getopts works.
    if name == "mod" {
        name = matches.free[2].clone()
    }

    if !utils::are_in_project() {
        println!("Please run this command inside a Cargo project. Exiting.");
        process::exit(1);
    }

    let mut current_dir = env::current_dir()
        .expect("Unexpected Error: Cannot get current working directory.");
    module::gen_module(name, private, &mut current_dir)
}

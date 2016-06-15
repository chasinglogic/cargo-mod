# cargo-mod 
A cargo sub command for generating modules.

----
![Build Status](https://travis-ci.org/ChasingLogic/cargo-mod.svg?branch=master)
[![Apache 2.0 License](https://img.shields.io/badge/license-Apache%202.0-ff69b4.svg)](https://github.com/ChasingLogic/cargo-mod/blob/master/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/cargo-mod.svg)](https://crates.io/crates/cargo-mod)
[![Crates.io](https://img.shields.io/crates/d/cargo-mod.svg)](https://crates.io/crates/cargo-mod)

----

## Installation
You can install cargo-mod using cargo itself with the following one-liner
```
cargo install cargo-mod
```

## Why
When writing Rust I grew very tired of creating modules by hand, it became especially tedious when I want to generate multiple nested modules and I would often forget to add an export to the mod.rs or to my own lib.rs etc etc, being a DevOps'er professionally whenever I see something that's manual, boring, and error-prone I can't help but want to automate it, so cargo-mod was born.

## Contributing
I'm always happy to accept pull requests for any features you would like to see added, a few I personally would like to see added are:

    - #5 Generators to remove boilerplate
      - I.e. I would like to generate Diesel.rs models, and nickel.rs / iron.rs routers/middleware etc.
    - #4 More intelligent addition of modstrings to mod.rs/lib.rs/main.rs
    - #3 Refactoring so unit testing is easier and code makes more sense.
    - #2 Improve integration tests so they are more thorough (i.e. check all parts of the generation.)

If you're not sure if your feature is a good fit or not, just submit a Github issue asking for comments before you start working on it!

As always follow the [Rust Code of Conduct](https://www.rust-lang.org/conduct.html), not only is it the nice thing to do it's one of the reasons I personally get so excited about Rust.

If something you add isn't covered by an existing integration test, please please please write one for your thing.

Please submit all pull requests to the develop branch.

## Building from source
Just like any Rust project you can simply build with ```cargo build``` however I've included a make file which adds some niceties for performing certain commands. Specifically when running tests you should always use ```make test``` since it will clean up and regenerate the cargo project used for integration testing.

## Usage
```
Create a new module or modules in the current cargo project.

Usage:
  cargo mod [<options>] [<path>]
  cargo mod -h | --help

Options:
  -h, --help        Print this message
  -p, --private     Make the generated module/s private

Details:
The path is a path seperated by / (even if on windows for now.) and will generate all folder modules
in between the final module and beginning module. The starting point being the current working directory.

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

We will also automatically add the correct mod exports to the generated mod.rs files AND to lib.rs/main.rs
whichever exists with a preference for lib.rs if both exist

However if you are inside the src/ directory we will start generation from your current directory in the repo.

If you want to only generate one module you can denote whether it is a folder or file module by the addition or omission
of a trailing /

Example folder:
cargo mod new/

Example file:
cargo mod new
```

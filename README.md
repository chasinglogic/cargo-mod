# cargo-mod 
A cargo sub command for generating modules.
----
![Build Status](https://travis-ci.org/ChasingLogic/cargo-mod.svg?branch=master)
[![Apache 2.0 License](https://img.shields.io/badge/license-Apache%202.0-ff69b4.svg)](https://github.com/ChasingLogic/cargo-mod/blob/master/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/cargo-mod.svg)](https://crates.io/crates/cargo-mod)
[![Crates.io](https://img.shields.io/crates/d/cargo-mod.svg)](https://crates.io/crates/cargo-mod)

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

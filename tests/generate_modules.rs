use std::fs::{File, remove_dir_all};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::process;
use std::env;

// All credit to: https://github.com/uutils/coreutils/blob/master/tests/common/macros.rs
macro_rules! path_concat {
    ($e:expr, ..$n:expr) => {{
        use std::path::PathBuf;
        let n = $n;
        let mut pb = PathBuf::new();
        for _ in 0..n {
            pb.push($e);
        }
        pb.to_str().unwrap().to_owned()
    }};
($($e:expr),*) => {{
    use std::path::PathBuf;
    let mut pb = PathBuf::new();
    $(
        pb.push($e);
    )*
        pb.to_str().unwrap().to_owned()
}};
}

fn get_executable_path() -> PathBuf {
    let mut cwd = env::current_dir().unwrap();
    cwd.push("target");

    cwd.push("release");
    if cwd.exists() {
        cwd.push("cargo-mod");
        return cwd
    }

    cwd.pop();
    cwd.push("debug");
    cwd.push("cargo-mod");
    return cwd
}

fn verify_generation(s: &mut String) {
    let mut p = env::current_dir().unwrap();
    p.push("tests");
    p.push("generator_test");
    p.push("src");

    if !s.ends_with("/") { s.push_str(".rs") }

    for d in s.split("/") {
        p.push(d);
        assert!(p.exists(), format!("Directory/File does not exist. {}", d));

        if !d.ends_with(".rs") {
            p.push("mod.rs");
            assert!(p.exists(), format!("Mod.rs does not exist for directory: {}", d));
            p.pop();
        }
    }

    // Cleanup
    loop {
        p.pop();
        if p.ends_with("src") {
            break
        }

        remove_dir_all(p.clone());
    }
}

#[test]
fn generate_multiple_modules() {
    let cmd = Command::new(get_executable_path())
        .arg("multiple/modules/test")
        .current_dir(path_concat!(env::current_dir().unwrap(), "tests", "generator_test"))
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    verify_generation(&mut "multiple/modules/test".to_string())
}

#[test]
fn generate_single_file_module() {
    let cmd = Command::new(get_executable_path())
        .arg("file")
        .current_dir(path_concat!(env::current_dir().unwrap(), "tests", "generator_test"))
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    verify_generation(&mut "file".to_string())
}

#[test]
fn generate_single_folder_module() {
    let cmd = Command::new(get_executable_path())
        .arg("folder/")
        .current_dir(path_concat!(env::current_dir().unwrap(), "tests", "generator_test"))
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    verify_generation(&mut "folder/".to_string())
}

#[test]
fn generate_private_module() {
    let cmd = Command::new(get_executable_path())
        .arg("-p")
        .arg("private")
        .current_dir(path_concat!(env::current_dir().unwrap(), "tests", "generator_test"))
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    let p = Path::new("tests/generator_test/src/lib.rs");
    let mut f = File::open(p).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    for line in s.lines() {
        if line == "pub mod private;" {
            assert!(false, "Found a public export of the private module.")
        }

        if line == "mod private;" {
            assert!(true)
        }
    }
}

#[test]
fn generate_module_inside_subdir() {
    // Generate the first folder
    let cmd = Command::new(get_executable_path())
        .arg("subfolder/")
        .current_dir(path_concat!(env::current_dir().unwrap(), "tests", "generator_test"))
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    verify_generation(&mut "subfolder/".to_string());

    let cmd = Command::new(get_executable_path())
        .arg("anotha/one")
        .current_dir(path_concat!(env::current_dir().unwrap(), "tests", "generator_test", "src", "subfolder"))
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    verify_generation(&mut "subfolder/anotha/one".to_string())
}

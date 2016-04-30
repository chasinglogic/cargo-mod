use std::path::PathBuf;
use std::fs;
use std::io::{Read, Write};
use super::project;

fn is_file(s: String) -> bool {
    if !s.ends_with("/") || s.ends_with(".rs") {
        return true
    }

    false
}

pub fn gen_module(mut name: String, private: bool) {
    let root_path = project::find_project_root();
    let mut our_path = root_path.clone();
    our_path.push("src");

    if !is_file(name) { name.push_str(".rs") }

    for dir in name.split("/") {
        if is_file(dir) {
            our_path.push(name);
            gen_file_module(root_path, our_path);
            break;
        }

        our_path.push(dir);
        gen_folder_module(root_path, &mut our_path.clone());
    }

    add_mod(&root_path, &mut our_path, generate_modstring(name, private))
}

fn gen_file_module(root_path: PathBuf, target_path: PathBuf) {
    let mut f = fs::File::create(target_path.as_path())
        .expect("Unable to create mod file.");

    f.write_all("".as_bytes())
        .expect("Unable to write to mod file.");

    println!("Created empty file: {}",
             super::pretty_path(&root_path, &target_path).display());

}

fn gen_folder_module(root_path: PathBuf, mut target_path: PathBuf) {
    fs::create_dir(target_path.as_path())
        .expect("Unable to create directory");

    println!("Created directory: {}", 
             super::pretty_path(&root_path, &target_path).display());

    target_path.push("mod.rs");
    let mut f = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(target_path.as_path())
        .expect("Unable to create mod.rs");

    // TODO: Is this necessary? Don't know if OpenOptions will create the file without being
    // written to.
    f.write_all("".as_bytes())
        .expect("Unable to create mod.rs");

    println!("Generated mod file: {}", 
             super::pretty_path(&root_path, &target_path).display())
}

fn generate_modstring(name: String, private: bool) -> String {
    if private {
        return format!("\nmod {};\n", &name);
    }

    format!("\npub mod {};\n", &name)
}

fn add_mod(root: &PathBuf, &mut target_path: PathBuf, name: String, private: bool) {
    let final_modstring = update_modrs(target_path, name, private);
    match project::kind_of_crate(&root) {
        project::CrateType::Both => {
            update_mainrs(root, &mut final_modstring.clone());
            update_librs(root, final_modstring)
        },
        project::CrateType::Library => update_librs(root, final_modstring),
        project::CrateType::Binary => update_mainrs(root, &mut final_modstring.clone()),
    }
}

fn update_modrs(target: PathBuf, name, private) -> String {
    
}

fn update_librs(root: &PathBuf, modstring: String) {
    let mut lib_path = root.clone();
    lib_path.push("src");
    lib_path.push("lib.rs");

    let mut librs = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(lib_path.as_path())
        .expect("Unable to open lib.rs");

    librs.write_all(modstring.as_bytes())
        .expect("Unable to update lib.rs")
}

fn update_mainrs(root: &PathBuf, modstring: &mut String) {
    let mut bin_path = root.clone();
    bin_path.push("src");
    bin_path.push("main.rs");

    let mut mainrs = fs::File::open(bin_path.as_path())
        .expect("Cannot open src/main.rs");
    let mut current_contents = String::new();
    mainrs.read_to_string(&mut current_contents)
        .expect("Cannot read from main.rs");

    let mut publess = modstring.trim_left_matches("pub").to_string();
    publess.push_str(current_contents.as_str());

    let mut new_file = fs::File::create(bin_path.as_path())
        .expect("Cannot update src/main.rs");
    new_file.write_all(publess.as_bytes())
        .expect("Unable to write to src/main.rs")
}


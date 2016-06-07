use std::path::PathBuf;
use std::fs;
use std::io::{Read, Write};
use std::env;

fn is_file(s: &str) -> bool {
    if !s.ends_with("/") || s.ends_with(".rs") {
        return true
    }

    false
}

pub fn gen_module(mut name: String, private: bool) {
    let mut working_dir = env::current_dir();

    // This makes sure that the name ends with .rs if not a directory
    if is_file(&name) { name.push_str(".rs") }

    for dir in name.split("/") {
        if is_file(&dir) {
            working_dir.push(name);
            gen_file_module(working_dir.clone());
            break;
        }

        working_dir = working_dir.push(dir).expect("Unexpected Error: Unable to push to working_dir");
        gen_folder_module(working_dir.clone());
        update_modrs(working_dir, generate_modstring(dir, private))
    }
}

fn gen_file_module(target_path: PathBuf) {
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

    // TODO: Is this necessary? I don't know if OpenOptions will create the file without being
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

// This function is definitely a feelsbadman.jpg
// There has got to be a better way to truncate the already open file...
// TODO: Investigate if this can be optimized to remove unnecessary disk IO
fn update_modrs(target: PathBuf, modstring: String) -> String {
    let modrs = what_to_update(&target);

    // Add this block so we destruct f when we are done with it
    {
        let mut f = fs::File::open(modrs.as_path())
            .expect(format!("Unable to open file: {}", modrs.display()));

        // Read all the contents of our target file
        let mut current_contents = String::new();
        f.read_to_string(&mut current_contents)
            .expect(format!("Unable to read from file: {}", modrs.display()));

        // Add our mod statement to top of the file
        modstring.push_str(current_contents.as_str());
    }

    let mut new_file = fs::File::create(modrs.as_path())
        .expect(format!("Cannot update file: {}", modrs.display()));
    new_file.write_all(modstring.as_bytes())
        .expect(format!("Cannot write to file: {}", modrs.display()));
}


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


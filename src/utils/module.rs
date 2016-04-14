pub fn generate_modstring(name: String, private: bool) -> String {
    if private {
        return format!("\nmod {};\n", &name);
    }

    format!("\npub mod {};\n", &name);
}

fn update_mainrs(root: &PathBuf, modstring: &mut String) {
    let mut bin_path = root.clone();
    bin_path.push("src");
    bin_path.push("main.rs");

    let mut mainrs = fs::File::open(bin_path.as_path()).unwrap();
    let mut current_contents = String::new();
    mainrs.read_to_string(&mut current_contents);

    modstring.push_str(current_contents.as_str());

    let mut new_file = fs::File::create(bin_path.as_path()).unwrap();
    match new_file.write_all(modstring.as_bytes()) {
        Ok(_) => println!("Updated main.rs"),
        Err(e) => println!("Unable to update main.rs: {}", e)
    }
}

fn update_librs(root: &PathBuf, modstring: String) {
    let mut lib_path = root.clone();
    lib_path.push("src");
    lib_path.push("lib.rs");

    let mut librs = fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(lib_path.as_path())
                .unwrap();

    match librs.write_all(modstring.as_bytes()) {
        Ok(_) => println!("Updated lib.rs"),
        Err(e) => println!("Unable to update lib.rs: {}", e)
    }
}

pub fn add_mod(root: &PathBuf, modstring: String) {
    match project::kind_of_crate(&root) {
        project::CrateType::Both => {
            update_mainrs(root, &mut modstring.clone());
            update_librs(root, modstring)
        },
        project::CrateType::Library => update_librs(root, modstring),
        project::CrateType::Binary => update_mainrs(root, &mut modstring.clone()),
    }
}

pub fn folder_module_exists(src_path: &PathBuf, name: String) -> bool {
    let mut check = src_path.clone();
    check.push(name);

    match fs::metadata(check) {
        Ok(_) => true,
        Err(_) => false,
    } 
}

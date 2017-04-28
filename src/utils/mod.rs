use std::env;

pub fn are_in_project() -> bool {
    let mut cwd = env::current_dir().unwrap();

    loop {
        cwd.push("Cargo.toml");

        if cwd.exists() {
            return true;
        }

        cwd.pop();
        if !cwd.pop() {
            return false;
        }
    }
}

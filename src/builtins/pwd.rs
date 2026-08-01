use std::env::current_dir;

pub fn run(_args: Vec<String>) -> bool {
    if let Ok(dir) = current_dir() {
        println!("{}", dir.to_string_lossy());
    }
    true
}

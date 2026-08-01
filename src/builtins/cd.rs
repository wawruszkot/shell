use std::{env, fs, path};

pub fn run(args: Vec<String>) -> bool {
    if !args.is_empty() {
        if args.len() != 1 {
            eprintln!("cd: too many arguments");
            return true;
        }

        let path = match args.first().unwrap().as_str() {
            "~" => env::home_dir().unwrap(),
            other => path::PathBuf::from(other),
        };
        match fs::metadata(&path) {
            Ok(metadata) => {
                if metadata.is_dir() {
                    env::set_current_dir(path).unwrap();
                } else {
                    eprintln!("cd: not a directory: {}", path.display());
                }
            }
            Err(_) => {
                eprintln!("cd: no such file or directory: {}", path.display());
            }
        }
    } else {
        let home_directory = env::var_os("HOME");

        if let Some(home_directory) = home_directory {
            env::set_current_dir(home_directory)
                .expect("Failed to find home directory");
        }
    }

    true
}

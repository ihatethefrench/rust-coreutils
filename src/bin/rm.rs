use std::{fs, path::Path, env, process::exit};
use lib::poem;

fn main() {
    for arg in env::args_os().skip(1) {
        let md = fs::metadata(&arg).unwrap_or_else(|_error| {
            println!("Error removing {:?}", &arg);
            exit(1);
        });
        let path = Path::new(&arg);
        if md.is_file() {
            fs::remove_file(path).unwrap();
        } else if md.is_dir() {
            fs::remove_dir_all(path).unwrap();
        } else {
            poem();
        }
    }
}

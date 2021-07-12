use std::{env, fs, path::Path, process::exit};

fn main() {
    for arg in env::args_os().skip(1) {
        let path = Path::new(&arg);
        let contents = fs::read_to_string(path).unwrap_or_else(|_error| {
            println!("Error reading {:?}", &arg);
            exit(1);
        });
        println!("# {}", path.display());
        println!("{}", contents);
    }
}

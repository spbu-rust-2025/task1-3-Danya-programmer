use std::fs::File;
use std::io;

fn main() {
    let mut path = String::new();
    let _ = io::stdin().read_line(&mut path);

    let path = path.trim();
    if path == "/" {
        println!("failure");
        return;
    }
    let normalized_path = path.trim_end_matches('/');
    if normalized_path.is_empty() || normalized_path == "/" {
        println!("failure");
        return;
    }
    let file: Result<File, io::Error> = File::open(path);
    println!("{}", path);
    match file {
        Ok(_) => {
            println!("success");
        }
        Err(_) => {
            println!("failure");
        }
    }
}

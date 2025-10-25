use std::fs;
use std::io;

fn main() {
    let mut path = String::new();
    let _ = io::stdin().read_line(&mut path);

    let path = path.trim_end_matches(&['\n', '\r'][..]);


    let result = fs::read_to_string(path);

    match result {
        Ok(_) => {
            println!("success");
        }
        Err(_) => {
            println!("failure");
        }
    }
}

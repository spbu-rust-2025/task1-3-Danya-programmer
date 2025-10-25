use std::fs::File;
use std::io;

fn main() {
    let mut path = String::new();
    let _ = io::stdin().read_line(&mut path);

    let path = path.trim_end_matches(&['\n', '\r'][..]);

    let file = File::open(path);

    match file {
        Ok(_) => {
            println!("success");
        }
        Err(_) => {
            println!("failure");
        }
    }
}

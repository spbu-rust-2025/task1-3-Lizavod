use std::fs::File;
use std::path::Path;

fn read_string() -> String {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string)
        .ok()
        .expect("failure");
    return string;
}

fn main() {
    let file_path = read_string();
    let path = Path::new(file_path.trim());
    let result = File::open(path);
    match result {
        Ok(_) => println!("success"),
        Err(_) => println!("failure")
    }
}

use std::fs;
use std::io;

fn read_string() -> String {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string)
        .ok()
        .expect("failure");
    return string;
}
fn read_file_contents(filename: &str) -> io::Result<String> {
    fs::read_to_string(filename)
}

fn main() {
    let file_path = read_string();
    match read_file_contents(file_path.trim()) {
        Ok(_) => println!("success"),
        Err(_) => println!("failure")
    }
}

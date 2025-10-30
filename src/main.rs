use std::fs::read_to_string;

use crate::util::ds::Obj;

mod util;

fn main() {
    for file in std::fs::read_dir("data").unwrap() {
        let file = file.unwrap();
        let path = file.path();
        let filename = file.file_name().to_string_lossy().to_string();

        if !filename.ends_with(".txt") {
            return;
        }
        println!("Detected {filename}");
        Obj::from_file_content(&read_to_string(path).unwrap());
    }
}

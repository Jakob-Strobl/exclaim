use std::fs::File;
use std::io::prelude::*;

pub fn read_file_to_string(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut expected = String::new();
    file.read_to_string(&mut expected).unwrap();
    let expected = expected.replace("\r\n", "\n"); // Remove incompatible newlines. damn you windows! 
    expected
}
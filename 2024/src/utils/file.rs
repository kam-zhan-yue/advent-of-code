use std::fs;

pub fn read_input(filename: &str) -> String {
    let path = format!("inputs/{}", filename);
    let error = format!("Could not find {path}");
    fs::read_to_string(path).expect(&error)
}

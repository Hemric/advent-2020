use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn load_input(path: &str) -> String {
    let mut file = File::open(Path::new(path)).expect("Error opening file");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error reading file");

    data.trim_end().to_string()
}

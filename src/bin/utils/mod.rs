use std::fs;
use std::io;

pub fn load_input(path: &str) -> Result<String, io::Error> {
    let data = fs::read_to_string(path)?;

    Ok(data.trim_end().to_string())
}

use std::{fs::read_to_string, io::Error};

pub fn read_file(path: String) -> Result<String, Error> {
    read_to_string(path)
}

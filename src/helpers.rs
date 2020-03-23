use std::fs;
use std::io;

pub fn read_file<'a>(filename: &'a str) -> io::Result<String> {
    let f = fs::read_to_string(filename)?;
    Ok(f)
}

#[derive(Debug, Clone)]
pub struct Pos {
    pub s: i64,
    pub e: i64
}

impl Pos {
    pub fn new(s: i64, e: i64) -> Pos {
        Pos { s, e }
    }
}

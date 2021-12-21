use std::io::{Read, BufRead, BufReader, Result};

pub fn read_line<R: Read>(reader: R) -> Result<String>
{
    let mut buffer = String::new();
    match BufReader::new(reader).read_line(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(error) => Err(error)
    }
}

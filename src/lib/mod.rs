use std::{fs::File, io::Write};

use error::HttpGettyError;

pub mod error;
pub mod core;
pub mod cli;



pub fn write_file(bytes: Vec<u8>, path: &str) -> Result<(), HttpGettyError> {
    let mut file = File::create(path)?;
    file.write_all(&bytes)?;
    Ok(())
}
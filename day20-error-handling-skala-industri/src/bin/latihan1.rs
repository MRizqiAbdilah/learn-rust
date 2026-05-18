use std::fs::read_to_string;
use anyhow::{Context, Result};

fn baca_token() -> Result<String> {
    let msg =
        read_to_string("token_rahasia.txt").context("File token tidak ditemukan di sistem")?;

    Ok(msg)
}

fn main() {
    let read_token = baca_token();

    match read_token {
        Ok(v) => println!("Hasil: {}", v),
        Err(e) => println!("Error: {:#?}", e),
    }
}

use anyhow::{Context, Result};

use thiserror::Error;

#[derive(Error, Debug)]

pub enum JaringanError {
    #[error("Sensor gagal merespons dalam {waktu} detik")]
    TimeOut { waktu: u32 },

    #[error("Sensor mengembalikan payload kosong")]
    DataKosong,
}

fn ping_sensor(waktu_tunggu: u32) -> Result<(), JaringanError> {
    if waktu_tunggu > 5 {
        return Err(JaringanError::TimeOut {
            waktu: waktu_tunggu,
        });
    }

    Ok(())
}

fn jalankan_sistem() -> Result<()> {
    ping_sensor(10).context("Sistem monitoring gagal beroperasi")?;
    Ok(())
}

fn main() {
    match jalankan_sistem() {
        Ok(_) => println!("Ping Sukses"),
        Err(e) => println!("Error: {:#?}", e),
    }
}

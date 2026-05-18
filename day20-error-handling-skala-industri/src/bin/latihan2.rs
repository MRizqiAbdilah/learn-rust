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

fn main() {
    match ping_sensor(10) {
        Ok(_) => println!("Ping Sukses"),
        Err(e) => println!("Error: {:#?}", e),
    }
}

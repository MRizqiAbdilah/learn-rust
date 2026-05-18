use anyhow::{Result, Context};
use thiserror::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncReadExt;
use serde_json::{self, from_str};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct PayloadSensor {
    id_node: String,
    kapasitas_persen: f32
}

#[derive(Debug, Error)]
enum GatewayError {
    #[error("Koneksi dari sensor terputus mendadak")]
    KoneksiTerputus,

    #[error("Format JSON tidak valid atau data rusak")]
    DataAnomali
}

async fn proses_sensor(mut socket: TcpStream) -> Result<()> {
    let mut buffer = [0;512];

    // Membaca stream...
    let ukuran = socket.read(&mut buffer).await?; // Gunakan ? jika gagal baca fisik
    if ukuran == 0 {
        return Err(GatewayError::KoneksiTerputus.into()); // .into() mengubahnya ke anyhow::Error
    }

    let string_data = String::from_utf8_lossy(&buffer[..ukuran]);

    // Gunakan ? di akhir map_err untuk melempar error ke atas
    let payload = from_str::<PayloadSensor>(&string_data)
        .map_err(|_| GatewayError::DataAnomali)?; 

    // Jika berhasil melewati baris di atas, berarti JSON valid!
    println!("Data Valid -> Node: {}, Kapasitas: {}%", payload.id_node, payload.kapasitas_persen);

    Ok(())
}


#[tokio::main]
async fn main(){
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server berjalan di http://127.0.0.1:8080");

    loop {
        let (socket, alamat_pengirim) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            match proses_sensor(socket).await {
                Ok(v) => println!("{:#?}", v),
                Err(e) => println!("{:#?}", e)
            }
        });
    }
}
use tokio::net::{TcpStream};
use tokio::io::AsyncWriteExt;

struct PayloadSensor {
    id_node: String,
    kapasitas_persen: f32
}

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();

    // Format string JSON mentah (gunakan r#""# agar tidak perlu escape character \")
    let payload_json = r#"{"id_node": "TPS-01", "kapasitas_persen": 75.5}"#;

    stream.write_all(payload_json.as_bytes()).await.unwrap();
    println!("Data berhasil dikirim ke Gateway!");
}
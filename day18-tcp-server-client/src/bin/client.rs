use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main(){
    let mut stream = TcpStream::connect("127.0.0.1:9000").await.unwrap();
    let payload = "NODE 1";

    stream.write_all(payload.as_bytes()).await.unwrap();
    println!("Data berhasil dikirim")
}
use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main(){
    let listener = TcpListener::bind("127.0.0.1:9000").await.unwrap();
    println!("Server berjalan di http://127.0.0.1:9000");

    loop {
        let (mut socket, alamat_pengirim) = listener.accept().await.unwrap();
        println!("Alamat masuk dari : {}", alamat_pengirim);
        let mut buffer = [0; 1024];

        match socket.read(&mut buffer).await {
                Ok(ukuran) if ukuran == 0 => return,
                Ok(ukuran) => {
                    let pesan = String::from_utf8_lossy(&buffer[..ukuran]);
                    println!("Data diterima: {}", pesan);
                },
                Err(err) => println!("Error membaca data: {}", err)
            }
    }
}
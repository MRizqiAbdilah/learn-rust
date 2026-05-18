#[tokio::main]
async fn main(){
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);
    let tx1 = tx.clone();

    tokio::spawn(async move {
        tx1.send("Data Suhu: 34C").await.unwrap();
    });

    tokio::spawn(async move {
        tx.send("Data Jarak: 80cm").await.unwrap();
    });

    while let Some(pesan) = rx.recv().await {
        println!("{pesan}");
    }

}
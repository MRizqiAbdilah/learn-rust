use std::{thread::JoinHandle, time::Duration};
use tokio::time::sleep;

use std::time::Duration;

#[tokio::main]
async fn main(){
    // Tipe datanya sekarang JoinHandle<String>
    let mut daftar_task: Vec<tokio::task::JoinHandle<String>> = vec![];

    for id_node in 1..=3 {
        let asisten = tokio::spawn(async move {
            println!("Node {} mengirim ping...", id_node);
            sleep(Duration::from_millis(500)).await;

            // Return sebuah nilai (tanpa titik koma)
            format!("Ping dari Node {} sukses", id_node) 
        });
        daftar_task.push(asisten);
    }

    for node in daftar_task {
        // Menunggu task selesai, dan menangkap nilai return-nya
        let hasil_return = node.await.unwrap(); 
        println!("Hasil: {}", hasil_return);
    }
}
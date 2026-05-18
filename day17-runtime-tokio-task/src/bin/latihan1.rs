use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main(){
    println!("Mereset Modul Jaringan...");
    sleep(Duration::from_millis(1000)).await;
    println!("Modul jaringan siap!");
}
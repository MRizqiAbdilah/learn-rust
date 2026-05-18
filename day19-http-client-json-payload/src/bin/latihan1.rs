use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Todo {
    userId: u32,
    id: u32,
    title: String,
    completed: bool
}

#[tokio::main]
async fn main(){ 
    let url = "https://jsonplaceholder.typicode.com/todos/1";

    // Ubah .text() menjadi .json::<Todo>()
    let response = reqwest::get(url).await.unwrap().json::<Todo>().await.unwrap();

    // Sekarang response adalah sebuah Struct Todo, bukan lagi String mentah
    println!("Tugas: {} | Status Selesai: {}", response.title, response.completed);
}
use reqwest::Client;
use serde::Serialize;

#[derive(Serialize)]
struct RekapHarian {
    id_gateway: String,
    total_sensor_aktif: u32,
    rata_rata_kapasitas: f32
}

#[tokio::main]
async fn main(){
    let url = "https://httpbin.org/post";
    let rekap = RekapHarian {id_gateway: String::from("1"), total_sensor_aktif: 100, rata_rata_kapasitas: 75.0};

    let client = Client::new();
    let respons = client.post(url).json(&rekap).send().await.unwrap();

    println!("Status Sinkronisasi: {}", respons.status());
    println!("Balasan dari server: \n{}", respons.text().await.unwrap())
}
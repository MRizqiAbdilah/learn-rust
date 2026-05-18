use futures::executor::block_on;

async fn baca_ultrasonik() -> u32 {
    return 85;
}

async fn baca_suhu() -> f32 {
    return 32.5;
}

async fn agregasi_sensor(){
    let ultrasonik = baca_ultrasonik().await;
    let suhu = baca_suhu().await;

    println!("Jarak: {} cm, Suhu: {} C", ultrasonik, suhu);
}

fn main() {
    let sensor = agregasi_sensor();
    block_on(sensor);
}


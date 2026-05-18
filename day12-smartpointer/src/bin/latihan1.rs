struct KonfigurasiDasar {
    id: u32,
    interval: f32,
    kalibrasi: f32,
}

fn tampilkan_id(config: Box<KonfigurasiDasar>) {
    println!("ID Sensor: [{}]", config.id)
}

fn main() {
    // dengan membungkusnya menggunakan Box::new() -> memindahkan data ke heap

    let konfigurasi: Box<KonfigurasiDasar> = Box::new(KonfigurasiDasar {
        id: 1,
        interval: 40.5,
        kalibrasi: 10.2,
    });

    tampilkan_id(konfigurasi); // valuenya akan dibawa di fungsi ini

    println!("{}", konfigurasi.id); // error -> karena tidak ada value yang dicetak
}

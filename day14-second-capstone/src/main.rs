use std::fs::{self, OpenOptions};
use std::io::Write;

struct DataSensor {
    id_node: u32,
    jarak_cm: f32
}

fn filter_dan_format(data_mentah: Vec<DataSensor>) -> Vec<String>{
    return data_mentah.into_iter().filter(|x| x.jarak_cm > 0.0 && x.jarak_cm <= 150.0).map(|x| format!("{},{}\n", x.id_node, x.jarak_cm)).collect();
}

fn simpan_ke_csv(data: Vec<String>) -> Result<(), String> {
    // 1. Buat folder (Instruksi yang terlewat)
    fs::create_dir_all("database_lokal").map_err(|_| "Gagal membuat direktori lokal")?;

    // 2. Buka file dengan map_err dan ?
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("database_lokal/logs.csv")
        .map_err(|e| format!("Gagal membuka file: {}", e))?; // Tangkap error tanpa Panic

    // 3. Tulis data
    file.write_all(data.join("").as_bytes())
        .map_err(|_| "Gagal menulis ke CSV")?;

    Ok(())
}

fn main(){
    let data: Vec<DataSensor> = vec![DataSensor{id_node:1, jarak_cm:100.0}, DataSensor{id_node:2, jarak_cm:140.0}, DataSensor{id_node:3, jarak_cm:-5.0}];

    let data_format = filter_dan_format(data);    
    match simpan_ke_csv(data_format) {
        Ok(_) => println!("Proses logging sukses! Data valid berhasil ditambahkan ke CSV."),
        Err(e) => println!("Sistem Peringatan: {}", e),
    }
}
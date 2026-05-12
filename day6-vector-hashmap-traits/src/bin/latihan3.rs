// Skenario: Gateway server Anda menerima berbagai macam format data struct, tapi hanya butuh satu cara standar untuk mengekstrak string payload-nya.
// Tugas:

// Buat trait bernama ProtokolPayload yang memiliki metode generate_json(&self) -> String.

// Buat dua struct: LogSampah (atribut: lokasi: String, persentase: f32) dan LogBaterai (atribut: tegangan: f32).

// Implementasikan trait ProtokolPayload untuk kedua struct tersebut.

// LogSampah akan me-return format string JSON: {"tipe": "sampah", "lokasi": "...", "persentase": ...}

// LogBaterai akan me-return format string JSON: {"tipe": "baterai", "v": ...}

// Buat variabel untuk masing-masing struct di fungsi main dan cetak hasil pemanggilan generate_json().

use serde::{Serialize, Deserialize};


pub trait ProtokolPayload {
    fn metode_json(&self) -> String;
}

#[derive(Serialize, Debug)]
struct LogSampah {
    lokasi: String,
    persentase: f32
}

#[derive(Serialize, Debug)]
struct LogBaterai {
    tegangan: f32
}

impl ProtokolPayload for LogSampah {
    fn metode_json(&self) -> String {
        let res = LogSampah {lokasi: self.lokasi.clone(), persentase: self.persentase.clone()};
        let serialized = serde_json::to_string(&res).unwrap();
        return serialized;
    }
}

impl ProtokolPayload for LogBaterai {
    fn metode_json(&self) -> String {
        let res = LogBaterai {tegangan: self.tegangan};
        let serialized = serde_json::to_string(&res).unwrap();
        return serialized;
    }
}

fn main() {
    let log_sampah: LogSampah = LogSampah { lokasi: String::from("Apalah"), persentase: 80.0 };
    let log_baterai: LogBaterai = LogBaterai { tegangan: 90.0 };

    let log_sampah1 = LogSampah::metode_json(&log_sampah);
    println!("log_sampah = {}", log_sampah1);

    let log_baterai1: String = LogBaterai::metode_json(&log_baterai);
    println!("log_sampah = {}", log_baterai1);


}
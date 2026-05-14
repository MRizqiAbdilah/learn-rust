// Skenario: Node Anda harus tahu batas ambang volume sampah dari sebuah file konfigurasi lokal, bukan di-hardcode di dalam program.
// Tugas:
    // Secara manual (pakai Notepad atau editor kode Anda), buatlah sebuah file bernama ambang_batas.txt di root folder proyek Anda.
    // Isi file tersebut hanya dengan satu angka: 75 (tanpa spasi atau karakter lain).
    // Tulis program Rust untuk membaca file ambang_batas.txt tersebut menggunakan fs::read_to_string.
    // Hapus karakter tak kasat mata (seperti spasi ekstra atau \n) dari string hasil bacaan menggunakan .trim(), lalu parsing menjadi tipe f32.
    // Cetak teks: "Konfigurasi dimuat. Batas peringatan di set ke: 75%" (Gunakan variabel hasil parsing Anda).

// Expected Output:
// Konfigurasi dimuat. Batas peringatan di set ke: 75%

use std::fs;

fn main(){
    let get_number = fs::read_to_string("ambang_batas.txt").expect("File tidak ditemukan");

    let clean_number = get_number.trim().parse::<f32>();

    println!("Konfigurasi dimuat. Batas peringatan di set ke: {:?}", clean_number);
}
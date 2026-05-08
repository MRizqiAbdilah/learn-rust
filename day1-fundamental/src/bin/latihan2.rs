// Anda menerima data mentah berupa total detik dari server: let waktu_mentah = 10000; (dalam integer).
// Tugas:
// Tanpa membuat variabel dengan nama baru selain waktu_mentah (Anda wajib menggunakan Shadowing berulang kali pada variabel ini), ubah total detik tersebut menjadi format string "Jam:Menit:Detik" dan cetak.
// (Catatan: Ini akan menguji pemahaman Anda tentang operator pembagian integer dan modulo, serta bagaimana merekayasa tipe data secara ekstrem dalam satu nama variabel).

use std::fmt::format;

fn main(){
    let waktu_mentah = 10000;

    let (mut jam, mut menit, mut detik) = (0,0,0);

    jam = waktu_mentah / 3600;

    if waktu_mentah % 3600 > 0 {
       menit = (waktu_mentah % 3600) ;
    }
    println!("Menit : {menit}");

    let final_menit = menit / 60;

    detik = menit % 60;

    println!("Jam : {jam}");
    println!("Menit : {final_menit}");
    println!("Detik : {detik}");

    let format_jam = jam.to_string();
    let format_menit = final_menit.to_string();
    let format_detik = detik.to_string();

    let format = format!("{}:{}:{}", format_jam, format_menit, format_detik);

    println!("{format}")
}
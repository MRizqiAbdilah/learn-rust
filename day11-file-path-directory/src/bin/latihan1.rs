// Skenario: Anda perlu menyimpan status booting pertama kali dari gateway IoT Anda.
// Tugas:
// Buat sebuah string berbunyi: "SYSTEM BOOT: OK\nMODUL SENSOR: ONLINE\nKONEKSI: MENUNGGU...".
// Gunakan perintah fs::write untuk menyimpan string tersebut ke dalam file bernama sistem_log.txt.
// Jika berhasil, cetak "Log sistem berhasil dibuat!" ke terminal. Setelah program dijalankan, cek folder project Anda apakah file tersebut benar-benar muncul!

// Expected Output di Terminal: Log sistem berhasil dibuat!

// Expected Output di dalam file sistem_log.txt:
// (3 baris teks sesuai string di atas)

use std::fs;

fn main(){
    let pesan = "SYSTEM BOOT: OK\nMODUL SENSOR: ONLINE\nKONEKSI: MENUNGGU...";

    fs::write("sistem_log.txt", pesan).expect("Gagal menulis file");
    println!("Log sistem berhasil dibuat!");

    let log = fs::read_to_string("sistem_log.txt").expect("Filenya gak boss");

    println!("{}", log)
}
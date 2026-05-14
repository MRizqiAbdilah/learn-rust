// Skenario: Ini adalah inti dari penyimpanan lokal. Anda akan mencatat data harian ke format .csv agar bisa dibuka di Excel. Data tidak boleh ditimpa!
// Tugas:
// Gunakan perintah Rust untuk membuat folder bernama rekaman_lokal.
// Anda memiliki data mentah ini di dalam variabel: let baris_baru = "1,RT_01,88.5\n";
// Tantangan Utama: Riset dan gunakan std::fs::OpenOptions beserta trait std::io::Write (Anda perlu melakukan use pada keduanya) untuk membuka file bernama log_harian.csv di dalam folder rekaman_lokal.
// Konfigurasikan OpenOptions agar file tersebut di-append (tambah di bawah) dan di-create (dibuat jika belum ada).
// Tulis baris_baru ke dalam file tersebut.
// Untuk mengujinya: Jalankan program Anda dua kali berturut-turut, lalu buka file log_harian.csv tersebut. Seharusnya ada dua baris data yang identik.

// Expected Output di Terminal: Data CSV berhasil ditambahkan!
// Expected Output di dalam file rekaman_lokal/log_harian.csv:

use std::fs;

fn main(){
    fs::create_dir_all("rekaman_lokal").expect("Gak bisa membuat directory");

    let data_mentah = "4,RT_01,89.5\n";

    let file = fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .append(true)
                .open("rekaman_lokal/log_harian.csv");

    let exist_file = fs::exists("rekaman_lokal/log_harian.csv").expect("Gak ada file yang ditemukan");
    
    let mut read_file = fs::read_to_string("rekaman_lokal/log_harian.csv").expect("Tidak ada filenya broo");

    println!("{}", read_file);

    if exist_file  && !read_file.is_empty() {
        read_file.push_str(data_mentah);
        fs::write("rekaman_lokal/log_harian.csv", read_file);
        println!("Data CSV berhasil ditambahkan!");
    } else {
        fs::write("rekaman_lokal/log_harian.csv", data_mentah);
        println!("File CSV berhasil di buat dan ditambahkan data!");
    }

}
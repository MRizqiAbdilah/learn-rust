// Skenario: Anda perlu melacak waktu (timestamp) kapan terakhir kali sebuah tempat sampah dibersihkan.
// Tugas:
// Buat sebuah HashMap di mana Key adalah ID Tempat Sampah (u32) dan Value adalah nama petugas terakhir yang mengambil sampah (String).
// Masukkan data untuk ID 1 ("Budi"), ID 2 ("Andi"), dan ID 3 ("Siti").
// Gunakan fungsi .entry().or_insert() (Anda harus melakukan Googling untuk ini!) untuk memasukkan data ID 2 ("Bambang") DAN ID 4 ("Joko").
// Terakhir, cetak seluruh isi HashMap menggunakan for (id, nama) in .... Perhatikan apakah nama petugas pada ID 2 berubah atau tidak!

use std::collections::{HashMap};

fn main(){
    let mut petugas: HashMap::<u32, &str> = HashMap::new();

    petugas.insert(1, "Budi");
    petugas.insert(2, "Andi");
    petugas.insert(3, "Siti");

    let remove_id: u32 = 2;
    petugas.remove(&remove_id);

    petugas.entry(2).or_insert("Bambang");
    assert_eq!(petugas[&2], "Bambang");

    petugas.entry(4).or_insert("Joko");
    assert_eq!(petugas[&4], "Joko");


    for (id, nama) in petugas {
        println!("ID: {id}, Petugas: {nama}")
    }
}
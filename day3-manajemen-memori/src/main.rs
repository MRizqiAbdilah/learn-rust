// Reviewer

// bin/latihan1.rs
// karena dengan menggunakan &str, bisa menerima banyak jenis string
// fn proses_data(id: u32, area: &str) {
//     println!("Memproses ID: {} di Area: {}", id, area);
// }

// fn main() {
//     let id_sensor = 404;
//     let lokasi: String = String::from("Kelurahan A");

//     // tambahkan & di lokasi, ini karena variabel lokasi itu String (Heap)
//     // jadi value nya itu tidak mengubah kepemilkikan karena mengambil dengan reference
//     proses_data(id_sensor, &lokasi);

//     // BAGAIMANA CARANYA AGAR BARIS DI BAWAH INI TIDAK ERROR?
//     // jadi pada println! kedua bisa memanggil lokasi karena value nya itu masih tetap dan tidak berubah kepemilikannya
//     println!("Selesai. Node {} tetap berada di {}", id_sensor, lokasi); 
// }


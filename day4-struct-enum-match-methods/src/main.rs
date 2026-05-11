// materials

// # Struct

// Mendefinisikan Struct
// struct SensorNode {
//     id: u32,
//     lokasi: String,
//     aktif: bool,
// }

// fn main() {
//     // Membuat instance (objek) dari Struct
//     let node1 = SensorNode {
//         id: 101,
//         lokasi: String::from("Sektor A"),
//         aktif: true,
//     };

//     println!("Node {} berada di {}", node1.id, node1.lokasi);
// }

// # Enum

// enum StatusKoneksi {
//     Online,
//     Offline,
//     Error(String), // Varian ini menyimpan pesan error bertipe String!
// }

// # Pattern Matching
// fn main() {
//     let status_node = StatusKoneksi::Error(String::from("Sinyal Hilang"));

//     match status_node {
//         StatusKoneksi::Online => println!("Sistem berjalan normal."),
//         StatusKoneksi::Offline => println!("Sistem sedang mati."),
//         StatusKoneksi::Error(pesan) => println!("Peringatan: {}", pesan),
//     }
// }

// # Methods & Associated Functions

// struct Kotak {
//     panjang: u32,
//     lebar: u32,
// }

// impl Kotak {
//     // Associated Function (Tanpa self) -> Mirip static method
//     fn new(p: u32, l: u32) -> Kotak {
//         Kotak { panjang: p, lebar: l }
//     }

//     // Method (Dengan &self) -> Membaca data
//     fn luas(&self) -> u32 {
//         self.panjang * self.lebar
//     }
// }

// fn main() {
//     let kotak_baru = Kotak::new(10, 5); // Memanggil Associated Function
//     println!("Luas kotak: {}", kotak_baru.luas()); // Memanggil Method
// }

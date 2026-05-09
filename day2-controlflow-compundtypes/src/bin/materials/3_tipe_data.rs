fn main() {
    // Tuple (id_node, suhu, aktif)
    let data_node: (u32, f32, bool) = (101, 32.5, true);
    
    // Mengambil nilai Tuple (Destructuring)
    let (id, suhu, status) = data_node; 
    println!("Node {} suhunya {}", id, suhu);

    // Array [tipe_data; jumlah_elemen]
    let riwayat_jarak: [u32; 5] = [120, 118, 115, 110, 105];
    println!("Jarak terakhir: {}", riwayat_jarak[4]); // Index dimulai dari 0
}
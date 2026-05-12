// Skenario: Anda ingin menghitung total berat sampah (estimasi) dari beberapa node yang aktif saja.
// Tugas:
// Buat struct NodeData dengan field aktif: bool dan berat_kg: f32.
// Buat sebuah vector yang berisi 4 instance NodeData:

// aktif: true, berat: 10.5
// aktif: false, berat: 5.0 (node mati, jangan dihitung)
// aktif: true, berat: 15.0
// aktif: true, berat: 4.5

// Gunakan Iterator untuk: menyaring (filter) node yang hanya aktif, lalu mengambil (map) nilai berat_kg-nya saja, dan terakhir menjumlahkan semuanya menggunakan method .sum::<f32>() (Anda tidak perlu .collect()). Simpan hasilnya di satu variabel dan cetak.


struct  NodeData {
    aktif: bool,
    berat_kg: f32
}

fn main(){
    let node: Vec<NodeData> = vec![NodeData {aktif: true, berat_kg: 10.5}, NodeData {aktif: false, berat_kg: 5.0}, NodeData {aktif: true, berat_kg: 15.0}, NodeData {aktif: true, berat_kg: 4.5}];

    let node_filter = node.iter().filter(|&x| x.aktif == true).map(|x| x.berat_kg / 4.0).sum::<f32>();
    
    print!("{node_filter}")
}
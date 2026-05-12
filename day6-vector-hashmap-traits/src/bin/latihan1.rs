// Skenario: Node IoT Anda hanya bisa menampung maksimal 5 log terakhir sebelum dikirim ke server.
// Tugas:
// Buat sebuah Vec<f32> kosong. Gunakan perulangan (for) untuk menambahkan 7 buah angka desimal sembarang ke dalam Vector tersebut (mensimulasikan data sensor yang masuk).
// Lalu, buat logika looping (bisa while atau metode lain) untuk terus melakukan .pop() selama panjang (length) dari Vector tersebut masih lebih dari 5.
// Terakhir, cetak isi Vector tersebut. Pastikan isinya hanya tersisa 5 angka pertama yang dimasukkan. (Petunjuk: cari method .len() pada Vector).
fn main () {

    let mut data_sensor: Vec<f32> = vec![12.5, 14.0, 13.5, 15.2, 11.0];

    while data_sensor.len() > 5 {
        data_sensor.pop();
    }

    print!("Isi buffer final: {:?}", data_sensor)
}
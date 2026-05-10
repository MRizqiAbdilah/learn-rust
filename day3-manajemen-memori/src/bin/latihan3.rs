fn main(){
    // String (Heap)
    let mut kapasitas_sampah = String::from("Sedang");

    // bukan menyalin. akan tetapi meminjam value dari kapasitas_sampah
    let baca1 = &kapasitas_sampah;
    let baca2 = &kapasitas_sampah;

    // Berhasil melakukan pengecekan value yaitu Sedang, baik baca1 dan baca2 itu sama
    println!("Pengecekan awal : {baca1}, {baca2}");

    // meminjam juga, akan tetapi bisa mengubah nilainya langsung dengan menggunakan &mut
    let ubah = &mut kapasitas_sampah;

    // Menambahkan string dengan push_str -> berhasil mengubah value dari kapasitas_sampah
    ubah.push_str(" menuju penuh");

    // kapasitas_sampah ikut berubah, karena variabel ubah ini memiliki wewenang mengubah valuenya juga
    println!("Status akhir tempat sampah : {kapasitas_sampah}")
}
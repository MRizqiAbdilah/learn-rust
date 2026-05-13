// Skenario: Anda ingin membuat fungsi utility yang bisa membandingkan dua nilai bacaan sensor untuk mencari yang paling tinggi, baik itu tegangan baterai (f32) maupun kuantitas peringatan (u32).
// Tugas:
// Buat sebuah fungsi fn sensor_tertinggi<T>(a: T, b: T) -> T. Fungsi ini harus me-return nilai yang lebih besar antara a dan b.
// ⚠️ TANTANGAN: Secara default, compiler tidak tahu apakah tipe T bisa dibandingkan dengan operator >. Anda wajib menambahkan Trait Bound PartialOrd pada T di deklarasi fungsi Anda (silakan Googling bagaimana cara menulis "Trait Bound PartialOrd di Rust").
// Uji fungsi tersebut di main dengan sepasang f32 (misal 88.5 dan 45.2) dan cetak.

fn sensor_tertinggi<T: PartialOrd>(a: T, b:T) -> T {
    if a > b { a } else {b}
}

fn main(){
    let( a, b )=( 88.5, 45.2);

    let hasil = sensor_tertinggi(a, b);

    println!("{}", hasil);
}
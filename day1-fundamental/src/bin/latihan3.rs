// Anda mendapatkan input data dimensi tempat sampah dari database berupa String Literal dan Integer secara acak:
    // let radius_str = "15";
    // let tinggi_int = 40;
    // Konstanta PI dengan nilai 3.14159.

// Tugas:
// Hitunglah volume tempat sampah silinder tersebut ($V = \pi \cdot r^2 \cdot h$).
// Syarat mutlak: Hasil akhir volume yang dicetak ke layar harus berupa tipe data integer u32 (dibulatkan ke bawah). Anda harus mencari tahu cara mengonversi string ke float, mengonversi integer ke float untuk kalkulasi agar tidak bentrok dengan PI, lalu mengonversinya kembali menjadi integer di akhir.

fn main(){
    let radius = "15";
    let tinggi: i32 = 40;
    const PI: f32 = 3.14159;

    let radius = radius.parse::<f32>().expect("Berupa angka!");
    let tinggi = tinggi as f32;

    let volume = PI * (radius * radius) * tinggi;

    let volume = volume.round() as u32;

    println!("Volume : {}", volume);
}
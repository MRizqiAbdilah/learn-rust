// Buatlah sebuah struct bernama TempatSampah.

// Berikan 3 atribut: id (u32), lokasi (String), dan volume_maksimal (f32).

// Buat blok impl TempatSampah dan tambahkan sebuah Associated Function bernama new yang menerima 3 parameter untuk mengisi atribut tersebut dan mengembalikan instance TempatSampah.

// Di fungsi main, buat variabel bin_a menggunakan TempatSampah::new(...) dengan data: ID 1, Lokasi "Jl. Mawar", Volume 100.0.

// Cetak detailnya ke layar

struct TempatSampah<'a> {
    id: u32,
    lokasi: &'a str,
    volume_maksimal: f32,
}

impl TempatSampah<'_> {
    fn new(id: u32, lokasi: &str, volume_maksimal: f32) -> TempatSampah<'_> {
        TempatSampah { id, lokasi, volume_maksimal }
    }

}

fn main(){
    let bin1 = TempatSampah::new(1, "Jl. Prumpung Sawah, 100.0", 100.0);

    println!("Registerasi berhasil: Bin #{} di {} (Kapasitas: {} liter)", bin1.id, bin1.lokasi, bin1.volume_maksimal)
}
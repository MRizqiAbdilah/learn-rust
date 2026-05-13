// Skenario: Gateway Anda perlu membungkus data sensor menjadi format standar sebelum dikirim, terlepas dari apakah datanya integer atau float.
// Tugas:
// Buat struct WadahData<T> yang memiliki atribut nilai: T.
// Buat blok implementasi impl<T> WadahData<T> dan tambahkan method ambil_nilai(&self) -> &T yang mengembalikan referensi dari atribut nilai.
// Di dalam main, buat dua instance: satu berisi data jarak bertipe i32 (1024), dan satu lagi berisi data persentase bertipe f64 (45.5).
// Cetak keduanya dengan memanggil ambil_nilai().

struct WadahData<T> {
    nilai: T
}

impl<T> WadahData<T> {
    fn ambil_nilai(&self) -> &T{
       &self.nilai
    }   
}

fn main(){
    let jarak_float: WadahData<f64> = WadahData { nilai: 45.5 };
    let jarak_int: WadahData<i32> = WadahData { nilai: 1024 } ;

    let jarak1 = WadahData::ambil_nilai(&jarak_float);
    let jarak2 = WadahData::ambil_nilai(&jarak_int);

    print!("Data ADC: {} | Data Kalibrasi: {}", jarak1, jarak2);
}
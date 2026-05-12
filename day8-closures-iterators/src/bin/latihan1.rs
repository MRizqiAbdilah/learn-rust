// Skenario: Sensor terkadang membaca nilai 0.0 (error pantulan) atau angka di atas 400.0 (di luar batas maksimal tong sampah).
// Tugas: Buat sebuah vector let log_jarak: Vec<f32> = vec![15.5, 0.0, 120.0, 450.5, 30.0, 0.0, 55.2];
// Gunakan Iterator dan Closure untuk membuat vector baru bernama data_valid yang hanya berisi angka yang lebih besar dari 0.0 DAN lebih kecil atau sama dengan 400.0. Cetak data_valid.

fn main(){
    let log_jarak: Vec<f32> = vec![15.5, 0.0, 120.0, 450.5, 30.0, 0.0, 55.2];

    // for i in 0..log_jarak.len(){
    //     if log_jarak[i] > 0.0 || log_jarak[i] <= 400.0 {
    //         print!("{}", log_jarak[i]);
    //     }
    // }

    let log_jarak_bersih: Vec<f32> = log_jarak.into_iter().filter(|&x| x > 0.0 && x <= 400.0 ).collect();

    print!("{:?}", log_jarak_bersih);
}
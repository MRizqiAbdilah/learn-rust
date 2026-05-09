// Node IoT Anda mendeteksi status kelembaban tanah dan merekamnya tiap detik dalam sebuah Array berkapasitas 10 elemen. Data berisi kata "Aman", "Waspada", dan "Kritis".
// let log_status = ["Aman", "Aman", "Waspada", "Aman", "Kritis", "Aman", "Aman", "Aman", "Aman", "Aman"];
// Tugas:
// Gunakan perulangan for dipadukan dengan .iter().enumerate() (silakan riset dokumentasinya!) untuk mendapatkan index dan nilainya sekaligus.
// Lakukan iterasi pada array tersebut:
    // Jika bertemu status "Waspada", tambahkan 1 ke variabel peringatan_count, lalu gunakan continue untuk langsung ke iterasi berikutnya.
    // Jika bertemu status "Kritis", segera hentikan perulangan seluruhnya menggunakan break.
    // Di akhir program, cetak jumlah peringatan, DAN buatlah sebuah Slice dari array awal yang hanya berisi data dari awal hingga tepat sebelum status "Kritis" itu muncul. Cetak slice tersebut ke layar.


fn main() {
    let log_status:[&str; 10]  = ["Aman", "Aman", "Waspada", "Aman", "Kritis", "Aman", "Aman", "Aman", "Aman", "Aman"];

    let mut peringatan_count = 0;


    for (i, val) in log_status.iter().enumerate() {
        let convert_val = val as &str;
        if convert_val == "Kritis" {
            break;
        } else if convert_val == "Waspada" {
            peringatan_count += 1;
            let log_status = &log_status[0..i];
            println!("{:?}", log_status);
            continue;
        } else {
            let log_status = &log_status[0..i];
            println!("{:?}", log_status);
        }

    }

    println!("Peringatan : {peringatan_count}")
}
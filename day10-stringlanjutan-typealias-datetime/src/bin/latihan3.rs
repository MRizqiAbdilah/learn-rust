// ### 🔥🔥🔥 Level 3: Sistem Pencatatan Waktu (Integrasi Chrono)
// **Skenario:** Setiap data yang masuk ke *database* harus memiliki stempel waktu (*timestamp*).
// **Tugas:**
// *(Pastikan Anda menjalankan `cargo add chrono` di terminal folder project Anda terlebih dahulu).*
// 1. Buat struct `DataMasuk` yang memiliki 2 *field*: `lokasi: String` dan `waktu: chrono::DateTime<chrono::Local>`.
// 2. Buat blok `impl` dengan *associated function* `new(lokasi_input: &str) -> DataMasuk`. Di dalam fungsi `new` ini, waktu harus otomatis di-isi dengan waktu sistem saat fungsi tersebut dipanggil.
// 3. Buat *method* `tampilkan_log(&self)` yang mencetak teks dengan format `"[{TANGGAL JAM}] Laporan dari {LOKASI}"`. Format waktunya wajib seperti ini: `DD/MM/YYYY - HH:MM:SS`.
// 4. Di `main`, buat objek baru dengan lokasi `"TPS RW 04"`, lalu panggil `tampilkan_log()`.

// * **Expected Output:** *(Waktu akan menyesuaikan jam di komputer Anda)*

use chrono::{DateTime, Local};

struct DataMasuk {
    lokasi: String,
    waktu: DateTime<Local>
}

impl DataMasuk {
    fn new(lokasi_input: String) -> DataMasuk{
        DataMasuk { lokasi: lokasi_input, waktu: Local::now() }
    }

    fn tampilkan_log(&self) {
        let waktu = self.waktu.format("%d/%m/%Y - %H:%M:%S");
        println!("[{}] Laporan dari {}", self.lokasi, waktu);
    }
}

fn main(){
    let log = DataMasuk::new(String::from("TPS RW 04"));
    
    log.tampilkan_log();
}

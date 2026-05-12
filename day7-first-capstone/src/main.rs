// 🛠️ Skenario Ujian: "Sistem Registrasi & Monitoring Gateway Lokal"
// Kebutuhan Sistem (Business Rules):

// Struktur Data (Struct & Enum):
// Buat enum StatusSensor dengan varian: Aktif, Offline, dan Error(String).
// Buat struct NodeTempatSampah. Harus memiliki ID (u32), lokasi (String), kapasitas batas (f32), dan status (StatusSensor).
// Implementasikan trait Reportable untuk NodeTempatSampah. Laporan harus merangkum ID, lokasi, dan statusnya dalam format string yang rapi.
// Penyimpanan (HashMap): Di fungsi main, Anda harus memiliki sebuah HashMap yang menggunakan ID sebagai Key dan objek NodeTempatSampah sebagai Value.
// Logika Penambahan Data (Error Handling & Borrowing):
// Buat sebuah fungsi bernama registrasi_node(...) -> Result<(), String>. Fungsi ini harus menerima mutable reference ke HashMap (&mut HashMap<u32, NodeTempatSampah>), ID, string lokasi (bebas &str atau String), dan kapasitas batas.
// Validasi: Jika kapasitas batas di bawah 50.0, fungsi harus me-return Err("Kapasitas terlalu kecil untuk standar kelurahan!").
// Jika valid, masukkan data ke dalam HashMap dengan status awal Aktif, lalu return Ok(()).
// Simulasi Alur Program di main:
// Inisialisasi HashMap kosong.
// Coba registrasi Node ID 1 ("RT 01", 120.0). Harus berhasil.
// Coba registrasi Node ID 2 ("RT 02", 40.0). Harus gagal dan cetak pesan errornya tanpa Panic!
// Coba registrasi Node ID 3 ("RT 03", 200.0). Harus berhasil.
// Gunakan manipulasi memori (mutable reference &mut) untuk langsung mengubah status Node ID 3 di dalam HashMap menjadi Error("Ultrasonik terhalang"). (Petunjuk: gunakan .get_mut(&3)).
// Terakhir, lakukan perulangan pada HashMap untuk mencetak hasil generate_report() dari setiap node yang berhasil terdaftar.

use std::{collections::HashMap, fmt::format};

pub trait Reportable {
    fn generate_report(&self) -> String;
}

#[derive(Debug)]
enum StatusSensor {
    None, 
    Aktif,
    Offline,
    Error(String)
}

pub struct NodeTempatSampah {
    id: u32,
    lokasi: String,
    kapasitas_batas: f32,
    status: StatusSensor,
}

fn registrasi_node(
   map: &mut HashMap<u32, NodeTempatSampah>,
    id: u32,
    lokasi: String,
    kapasitas_batas: f32
) -> Result<(), String> {
    if kapasitas_batas < 50.0 {
        return Err(String::from("Koneksi sensor terputus"));
    }
    let status: StatusSensor = StatusSensor::Aktif;
    map.insert(id, NodeTempatSampah {id, lokasi, kapasitas_batas, status});
    Ok(())
}

impl Reportable for NodeTempatSampah {
    fn generate_report(&self) -> String {
        match &self.status {
            StatusSensor::Aktif => format!("[ID: {:?}] Lokasi: {:?} | Status: {:?}", self.id, self.lokasi, self.status),
            StatusSensor::Error(_) => format!("[ID: {:?}] Lokasi: Error {:?} | Status:  {:?}", self.id, self.lokasi, self.status),
            StatusSensor::None => format!("[ID: {:?}] Lokasi: {:?} | Status: {:?}", self.id, self.lokasi, self.status),
            StatusSensor::Offline => format!("[ID: {:?}] Lokasi: {:?} | Status: {:?}", self.id, self.lokasi, self.status)
        }
        // format!("[ID: {:?}] Lokasi: {:?} | Status: {:?}", self.id, self.lokasi, self.status)
    }
}

fn main() {
    let mut map: HashMap<u32, NodeTempatSampah> = HashMap::new();
   
    let hasil_1 = registrasi_node(&mut map, 1, String::from("RT 11"), 60.0); 

    let hasil_2 = registrasi_node(&mut map, 2, String::from("RT 11"), 40.0); 
    
    let hasil_3 = registrasi_node(&mut map, 3, String::from("RT 11"), 200.0); 

    match hasil_1 {
        Ok(v) => println!("Berhasil mendaftarkan Node 1"),
        Err(pesan) => println!("{}", pesan)
    }   

    match hasil_2 {
        Ok(v) => println!("Berhasil mendaftarkan Node 1"),
        Err(pesan) => println!("{}", pesan)
    }   

    match hasil_3 {
        Ok(v) => println!("Berhasil mendaftarkan Node 3"),
        Err(pesan) => println!("{}", pesan)
    }   

    let update_id: u32 = 3;
    map.get_mut(&update_id).map(|e| *e = NodeTempatSampah { id: 3, lokasi: String::from("123"), kapasitas_batas: 100.0, status: StatusSensor::Error(String::from("- Ultrasonik terhalang")) });

    for (id, node) in &map {
        let report = Reportable::generate_report(node);
        println!("{}", report)
    }
}

// Gabungkan konsep Level 1 dan Level 2. Tambahkan atribut baru pada struct TempatSampah yaitu status yang bertipe KondisiSampah (pastikan enum sudah didefinisikan di atasnya).
// Perbarui fungsi new agar setiap tempat sampah yang baru dibuat memiliki status default KondisiSampah::Kosong.
// Buat sebuah Method bernama update_sensor(&mut self, persentase_baru: f32).
// Di dalam method update_sensor, ubah nilai self.status menggunakan logika if/else:
    // Jika persentase_baru >= 90.0, ubah status menjadi Penuh.
    // Jika persentase_baru == 0.0, ubah status menjadi Kosong.
    // Selain itu, ubah menjadi Terisi(persentase_baru).
// Buat juga method cek_status(&self) yang menggunakan match pada self.status untuk mencetak kalimat peringatan seperti pada Level 2.
// Di fungsi main, buat instance tempat sampah (harus mut), lalu simulasikan perubahan data: panggil method cek_status(), lalu update_sensor(45.0), panggil cek_status() lagi, lalu update_sensor(95.0), dan panggil cek_status() lagi.
enum KondisiSampah {
    Kosong,
    Terisi(f32),
    Penuh
}
struct TempatSampah<'a> {
    id: u32,
    lokasi: &'a str,
    volume_maksimal: f32,
    status: KondisiSampah
}

impl TempatSampah<'_> {
    fn new(id: u32, lokasi: &str, volume_maksimal: f32, status: Option<KondisiSampah>) -> TempatSampah<'_> {
        let status = status.unwrap_or(KondisiSampah::Kosong);
        TempatSampah { id, lokasi, volume_maksimal, status }
    }

    fn update_sensor(&mut self, persentase_baru: f32) {
        if persentase_baru >= 90.0 {
            self.status = KondisiSampah::Penuh;
        } else if persentase_baru == 0.0 {
            self.status = KondisiSampah::Kosong;
        } else {
            self.status = KondisiSampah::Terisi(persentase_baru);
        }
    }

    fn check_status(&self) {
        match self.status {
        KondisiSampah::Kosong => println!("Aman, Tidak perlu pengangkutan"),
        KondisiSampah::Terisi(state) => println!("Terisi {}%. lanjutkan monitoring", state),
        KondisiSampah::Penuh => println!("KIRIM TRUK PENGANGKUT SEKARANG!")
        }
    }
}

fn main(){ 
    let mut bin = TempatSampah::new(1, "Jl. Prumpung Sawah", 95.5, None);

    print!("Status Bin {}: ", bin.lokasi);
    bin.check_status();

    bin.update_sensor(45.0);
    print!("Status Bin {}: ", bin.lokasi);
    bin.check_status();

    bin.update_sensor(95.5);
    print!("Status Bin {}: ", bin.lokasi);
    bin.check_status();

}
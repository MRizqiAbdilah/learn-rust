// Dalam optimalisasi pengangkutan, truk sampah tidak perlu datang jika sampah masih sedikit.
// Tugas:
// Buat enum bernama KondisiSampah.
// Buat 3 varian: Kosong, Terisi(f32) (varian ini menyimpan persentase isinya), dan Penuh.
// Di fungsi main, buat variabel penampung state saat ini: let state_sekarang = KondisiSampah::Terisi(65.5);
// Gunakan match untuk mengevaluasi state_sekarang:
    // Jika Kosong, cetak: "Aman, tidak perlu pengangkutan."
    // Jika Terisi(nilai), cetak: "Terisi [nilai]%. Lanjutkan monitoring."
    // Jika Penuh, cetak: "KIRIM TRUK PENGANGKUT SEKARANG!"

enum KondisiSampah {
    Kosong,
    Terisi(f32),
    Penuh
}

fn main(){
    let state_sekarang = KondisiSampah::Terisi(65.5);

    match state_sekarang {
        KondisiSampah::Kosong => println!("Aman, Tidak perlu pengangkutan"),
        KondisiSampah::Terisi(state) => println!("Terisi {}%. lanjutkan monitoring", state),
        KondisiSampah::Penuh => println!("KIRIM TRUK PENGANGKUT SEKARANG!")
    }
}
use futures::executor::block_on;

async  fn unduh_konfigurasi() {
    println!("Mengunduh dari server...");
}

fn main() {
    let task = unduh_konfigurasi();

    println!("Fungsi main sedang berjalan");
    block_on(task);
}
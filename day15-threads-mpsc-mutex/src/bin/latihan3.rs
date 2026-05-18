use std::{sync::{Arc, Mutex}, thread};

fn main(){
    // Menggunakan integer sederhana, bukan Vector
    let total_terangkut: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut kumpulan_thread = vec![];

    for _ in 0..3 { // Looping 3 kali
        let clone_total = Arc::clone(&total_terangkut);
        let asisten = thread::spawn(move || {
            let mut data = clone_total.lock().unwrap();
            *data += 500; // Melakukan operasi matematika pada nilai di dalam dereference (*)
        });
        kumpulan_thread.push(asisten);
    }

    for asisten in kumpulan_thread {
        asisten.join().unwrap();
    }

    // Cetak hasil akhirnya
    println!("Total Akhir : {} kg", *total_terangkut.lock().unwrap());
}
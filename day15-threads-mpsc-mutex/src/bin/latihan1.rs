use std::{thread, time::Duration};

fn main() {
    let tugas = vec![1, 2, 3, 4, 5];
    let value = tugas.clone();
    let asisten = thread::spawn(move || {
        for i in 0..value.len() - 1 {
            println!("Asisten memproses data: {}", value[i]);
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in tugas {
        println!("Utama Memproses UI: {}", i);
        thread::sleep(Duration::from_millis(100));
    }

    asisten.join().unwrap();
}

use std::{sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    let node1 = thread::spawn(move || {
        let payload = String::from("Node 1: 85%");
        tx.send(payload).unwrap();
    });

    let node2 = thread::spawn(move || {
        let payload = String::from("Node 2: Penuh");
        tx1.send(payload).unwrap();
    });

    for pesan in rx {
        println!("Gateway menerima pesan: {}", pesan);
    }
}
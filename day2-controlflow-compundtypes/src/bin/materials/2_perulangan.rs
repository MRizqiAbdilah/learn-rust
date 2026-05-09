fn main() {
    // 1. loop: Berjalan selamanya sampai ada instruksi `break`
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 3 { break; } // Menghentikan paksa
    }

    // 2. while: Berjalan selama kondisi true
    let mut baterai = 100;
    while baterai > 0 {
        baterai -= 50; 
    }

    // 3. for in: Paling aman dan disarankan di Rust (tidak rawan index out of bounds)
    for angka in 1..=3 { // 1..=3 berarti rentang 1 sampai 3
        println!("Iterasi ke-{}", angka);
    }
}
fn main() {
    let kapasitas = 85;
    
    // Cara standar
    if kapasitas >= 90 {
        println!("Tempat sampah hampir penuh!");
    } else if kapasitas >= 50 {
        println!("Terisi setengah.");
    } else {
        println!("Masih kosong.");
    }

    // Rust Way: if sebagai expression (mirip ternary operator)
    let status_led = if kapasitas > 80 { "MERAH" } else { "HIJAU" };
    println!("Lampu indikator: {}", status_led);
}
fn main() {
    let semua_bacaan = [10, 20, 30, 40, 50];
    
    // Mengambil elemen dari index 1 hingga 3 (index 4 tidak ikut)
    let potongan: &[i32] = &semua_bacaan[1..4]; 
    println!("Potongan data: {:?}", potongan); // Output: [20, 30, 40]
}
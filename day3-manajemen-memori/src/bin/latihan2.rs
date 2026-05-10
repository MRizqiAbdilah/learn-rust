// parameter memiliki type &mut f32, ini tuh bakal meminjam value akan tetapi memiliki hak untuk mengubahnya juga
fn update_kalibrasi(nilai: &mut f32) {
    // *nilai -> * itu akan dereference karena kalo tanpa itu tidak bisa, ingkat karena tipe nilai ini menggunakan &mut f32 
    *nilai += 2.5;
}

fn main(){
    //  mutable -> bisa diubah valuenya
    let mut jarak_kalibrasi: f32 = 12.5;

    // melakukan perubah nilai/value dari variabel jarak_kalibrasi
    update_kalibrasi(&mut jarak_kalibrasi);

    println!("Jarak kalibrasi baru : {jarak_kalibrasi}");

}
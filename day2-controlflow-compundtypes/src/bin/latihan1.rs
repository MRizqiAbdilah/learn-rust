// Sebuah gateway menerima paket data dari sensor dalam bentuk Tuple (String, u32, bool). Format datanya adalah (Lokasi, Persentase_Kapasitas, Status_Baterai_Lemah).
// Tugas:
// Buatlah variabel Tuple dengan nilai ("Sektor B", 92, true). Ekstrak Tuple tersebut menggunakan teknik destructuring. Lalu buat logika percabangan:

// Jika kapasitas di atas 90 DAN baterai lemah (true), cetak peringatan darurat untuk segera melakukan pengangkutan dan ganti baterai.

// Jika hanya kapasitas di atas 90, cetak peringatan pengangkutan.

// Jika hanya baterai lemah, cetak peringatan baterai.

// Selain itu, cetak "Aman".

fn main(){
    let data_node: (&str, u8, bool) = ("Sektor B", 92, true);

    let (_lokasi, persentase_kapasitas, status_baterai_lemah) = data_node;


    if persentase_kapasitas > 90 && status_baterai_lemah == true {
        println!("Segera melakukan pengangkutan dan ganti baterai");
    } else if persentase_kapasitas > 90 {
        println!("Segera melakukan pengangkutan")
    } else {
        println!("Aman");
    }
}
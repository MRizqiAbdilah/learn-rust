// Sebuah node sensor IoT menggunakan baterai berkapasitas 3300 mAh. Setiap kali sensor mengirim data, ia menghabiskan 15 mAh. Sensor tersebut juga memiliki kebocoran daya statis sebesar 2 mAh per jam.
// Tugas: Buatlah program yang memiliki sebuah variabel konstanta kapasitas baterai. Lalu, asumsikan sensor telah berjalan selama 24 jam dan mengirim data sebanyak 40 kali. Hitung sisa baterai saat ini, lalu cetak hasilnya ke layar. Pastikan Anda menggunakan tipe data unsigned integer yang tepat.
fn main(){
    
    const CAPACITY_BATTERY: u32 = 3300;

    let consume:u32 = 15;

    let total_consume: u32 = consume * 40;

    let remaining_battery = CAPACITY_BATTERY - total_consume;

    print!("Sisa baterai saat ini : {remaining_battery}")

}
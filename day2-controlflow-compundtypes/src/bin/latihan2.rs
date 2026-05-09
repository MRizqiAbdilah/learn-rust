// Sensor ultrasonik di lapangan sering kali memberikan nilai pantulan yang salah (anomali) akibat lalat atau debu. Anda memiliki sekumpulan data pembacaan jarak (dalam cm):
// let data_sensor: [f32; 8] = [150.0, 148.5, 149.0, 12.0, 148.0, 151.0, 150.5, 9.5];
// Tugas:
// Nilai 12.0 dan 9.5 jelas merupakan anomali karena perubahannya terlalu drastis. Buatlah program menggunakan loop untuk menghitung rata-rata jarak, TETAPI abaikan nilai yang berada di bawah 50.0. Cetak hasil rata-rata yang sudah dibersihkan dari noise tersebut. (Anda akan membutuhkan variabel penampung total jumlah dan penampung jumlah iterasi yang valid).

fn main(){
    let data_sensor: [f32; 8] = [150.0, 148.5, 149.0, 12.0, 148.0, 151.0, 150.5, 9.5];

    let mut total: f32 = 0.0;

    for data in data_sensor {
        println!("{data}");
        if data < 50.0 {
            println!(" karena {data} di bawah 50, maka terindikasi anomali, jangan ditambahkan")
        } else {
            total += data;
        }
    }

    // let banyak_pantulan = data_sensor.len() as f32;
    let rata_rata = total / (data_sensor.len() as f32);
    println!("Rata-Rata Nilai Pantulan Sensor ultrasonik : {rata_rata}");
}
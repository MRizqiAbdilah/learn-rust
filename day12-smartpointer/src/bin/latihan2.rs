use std::rc::Rc;

struct StrukLaporan<'a> {
    message: &'a str,
    data: Rc<f32>,
}

struct StrukAnalisis<'a> {
    message: &'a str,
    data: Rc<f32>,
}

fn main() {
    // instance variabel bertipe Rc<f32>
    let data_jarak: Rc<f32> = Rc::new(15.5);

    // Clone dengan Rc::clone() -> membuat data_jarak dimiliki oleh banyak pemilik dalam satu thread
    let struk_laporan = StrukLaporan {
        message: "Data yang dikirimkan oleh Struk Laporan",
        data: Rc::clone(&data_jarak),
    };

    let struk_analisis = StrukAnalisis {
        message: "Data yang dikirimkan oleh Struk Analisis",
        data: Rc::clone(&data_jarak),
    };

    // menghitung seberapa banyak pemilik (variabel asal + 2 struct)
    println!("Jumlah pemilik data: {}", Rc::strong_count(&data_jarak));
    println!("{}: {}", struk_laporan.message, struk_laporan.data);
    println!("{}: {}", struk_analisis.message, struk_analisis.data);
}

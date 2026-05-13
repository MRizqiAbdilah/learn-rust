// Skenario: Anda menyimpan string nama area operasional di memori utama, lalu ingin membagikan referensinya ke dalam struct log agar hemat memori.
// Tugas:
// Buat struct LogOperasional<'a> dengan atribut area: &'a str.
// Buat fungsi implementasinya yang memiliki method cetak_log(&self).
// Di main, buat sebuah variabel String::from("Kelurahan Kebon Manggis") (jangan langsung string literal, harus ditaruh di String dari heap).
// Buat instance LogOperasional dan pinjamkan (&) isi dari String tersebut ke atribut area.
// Panggil cetak_log().

#[derive(Debug)]
struct LogOperasional<'a> {
    area: &'a str
}

impl<'a> LogOperasional<'a> {
    fn cetak_log(&self) {}
}

fn main(){
    let kelurahan  = String::from("Kelurahan Kebon Manggis");

    let area = LogOperasional{area: &kelurahan};

    LogOperasional::cetak_log(&area);

    println!("{:?}", area);
}


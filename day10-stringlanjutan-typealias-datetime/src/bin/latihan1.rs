type IDPerangkat = u64;
type PesanLog = String;

fn catat_aktivitas(id: IDPerangkat, pesan: &str){
    println!("Aktivitas [{}]: {}", id, pesan);
}

fn main(){
    let id_truk: IDPerangkat = 909;
    let laporan: PesanLog = String::from("Sedang menuju TPA");

    catat_aktivitas(id_truk, &laporan);
}
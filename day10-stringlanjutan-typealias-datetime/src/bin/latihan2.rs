fn cetak_data(id:&str , kapasitas: f32, status: &str){
    println!("Data Bersih -> ID: {} | Terisi: {} | Status: {}", id, kapasitas, status);
}

fn main(){
    let payload_mentah = String::from("   NODE-A, 85.5 , WARNING   ");
    let payload: Vec<_> = payload_mentah.trim().split(",").collect();

    
    let (id_node, kapasitas, status) = (payload[0], payload[1], payload[2]);
    let remove_whitespace = kapasitas.trim();

    let kapasitas: f32 = remove_whitespace.parse::<f32>().expect("Must a valid number");
    println!("{}", kapasitas);
    cetak_data(id_node, kapasitas, status);
}
fn main(){
    let ruang_kosong: Vec<f32> = vec![80.0, 50.0, 10.0, 100.0];

    let ruang_kosong_persentase: Vec<f32> = ruang_kosong.iter().map(|x| (100.0 - x)/100.0 * 100.0).collect();

    print!("{:?}", ruang_kosong_persentase);

}
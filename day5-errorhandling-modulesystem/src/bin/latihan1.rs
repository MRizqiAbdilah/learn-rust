fn parsing_baterai(input: &str) -> Result<u32, String> {
    let parse_input = input
        .parse::<u32>()
        .map_err(|_| format!("Peringatan: Gagal membaca data baterai!"));

    match parse_input {
        Ok(val) => {
            println!("Baterai: {val}%");

            Ok(val)
        }

        Err(e) => Err(e),
    }
}

fn main() {
    let baterai = "85";

    let _ = parsing_baterai("10");
}

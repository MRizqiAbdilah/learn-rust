fn validasi_jarak(input: &str) -> Result<u32, String> {
    let parse_input = input
        .parse::<u32>()
        .map_err(|_e| format!("Gagal konversi string ke angka"))?;

    if parse_input > 200 {
        return Err(String::from("Jarak di luar jangkauan sensor"));
    }

    Ok(parse_input)
}

fn main() {
    let validisi1 = validasi_jarak("150");

    let validisi2 = validasi_jarak("250");

    let validisi3 = validasi_jarak("abc");

    match validisi1 {
        Ok(v) => println!("Data valid: {v} cm"),

        Err(err) => println!("{}", err),
    }

    match validisi2 {
        Ok(v) => println!("Data valid: {v} cm"),

        Err(err) => println!("{}", err),
    }

    match validisi3 {
        Ok(v) => println!("Data valid: {v} cm"),

        Err(err) => println!("{}", err),
    }
}

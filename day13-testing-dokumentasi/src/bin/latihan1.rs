fn kalkulasi_sisa_jarak(jarak_sensor: f32, tinggi_tong: f32) -> f32 {
    tinggi_tong - jarak_sensor
}

#[cfg(test)]
mod tests {
    use crate::kalkulasi_sisa_jarak;

    #[test]
    fn uji_kalkulasi_normal() {
        let hasil: f32 = kalkulasi_sisa_jarak(30.0, 100.0);
        assert_eq!(hasil, 70.0);
    }
}

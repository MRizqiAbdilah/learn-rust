fn kalkulasi_sisa_jarak(jarak_sensor: f32, tinggi_tong: f32) -> bool {
    jarak_sensor > tinggi_tong
}

#[cfg(test)]

mod tests {
    use crate::kalkulasi_sisa_jarak;
    #[test]
    #[should_panic]
    fn uji_jarak_berlebih() {
        let hasil: bool = kalkulasi_sisa_jarak(100.0, 30.0);

        assert_eq!(hasil, true);
    }
}

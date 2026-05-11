pub mod sistem_kontrol {

    pub struct TrukPengangkut {
        pub id: u32,

        kapasitas_maks: u32,
    }

    impl TrukPengangkut {
        pub fn new(id: u32, kapasitas_maks: u32) -> TrukPengangkut {
            TrukPengangkut { id, kapasitas_maks }
        }

        pub fn update_id(&mut self, input: u32) {
            self.id = input;
        }

        pub fn display(&self) {
            println!(
                "Truk #{} dikerahkan (Kapasitas: {} kg)",
                &self.id, &self.kapasitas_maks
            )
        }
    }
}

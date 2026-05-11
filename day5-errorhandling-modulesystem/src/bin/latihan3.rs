mod node;

fn main() {
    let mut truk = crate::node::sistem_kontrol::TrukPengangkut::new(1, 5000);

    truk.display();

    truk.update_id(999);

    truk.display();
}

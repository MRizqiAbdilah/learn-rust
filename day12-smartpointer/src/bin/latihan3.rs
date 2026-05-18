use std::{cell::RefCell, rc::Rc};
type SharedData = Rc<RefCell<u32>>;

fn main() {
    let kapasitas_sampah: SharedData = Rc::new(RefCell::new(40));
    let tampilan_dashboard = Rc::clone(&kapasitas_sampah);
    let pembaca_sensor = Rc::clone(&kapasitas_sampah);

    *pembaca_sensor.borrow_mut() = 90;

    println!("Status Dashboard: {}%", tampilan_dashboard.borrow());
}

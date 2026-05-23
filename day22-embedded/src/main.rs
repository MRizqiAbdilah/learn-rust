#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println; // Gunakan println biasa yang sangat simpel
use esp_hal::{
    delay::Delay, 
    entry, // Ini menyelesaikan error "cannot find attribute entry"
    gpio::{Io, Level, Output}
};
// Gunakan standar embedded-hal versi terbaru
use embedded_hal::digital::OutputPin; 

#[derive(Debug, PartialEq, Clone, Copy)]
enum StatusPompa {
    Aktif,
    Mati,
}

struct PengontrolPompa<P> {
    pin: P,
}

struct DataHidroponik {
    suhu: f32,
    kelembaban: f32,
    level_ph: f32,
    status_pompa: StatusPompa,
}

impl DataHidroponik {
    fn new(suhu: f32, kelembaban: f32, level_ph: f32, status_pompa: StatusPompa) -> Self {
        DataHidroponik { suhu, kelembaban, level_ph, status_pompa }
    }

    fn evaluasi_ph(&self) -> &str {
        if self.level_ph >= 5.5 && self.level_ph <= 6.5 {
            "pH Normal"
        } else {
            "pH Bahaya"
        }
    }
}

impl<P: OutputPin> PengontrolPompa<P> {
    fn new(pin: P) -> Self {
        PengontrolPompa { pin }
    }

    fn nyalakan(&mut self) {
        let _ = self.pin.set_high(); 
    }

    fn matikan(&mut self) {
        let _ = self.pin.set_low();
    }
}

fn loop_hidroponik<P: OutputPin>(
    data: &DataHidroponik, 
    pengontrol_pompa: &mut PengontrolPompa<P>,  
    delay: &mut Delay
) {
    // Tambahkan \r di akhir string sebelum tanda kutip penutup
    log::info!("--- Siklus Pembacaan ---\r");
    log::info!("Suhu: {} °C | Kelembaban: {} %\r", data.suhu, data.kelembaban);
    log::info!("pH saat ini: {}\r", data.level_ph);

    let status = data.evaluasi_ph();
    log::info!("Status: {}\r", status);

    if status == "pH Bahaya" {
        log::info!("[AKSI] Pompa NYALA (3 detik)...\r");
        pengontrol_pompa.nyalakan();
        
        delay.delay_millis(3000); 
        
        log::info!("[AKSI] Pompa MATI.\r");
        pengontrol_pompa.matikan();
    } else {
        log::info!("[AKSI] Aman. Pompa MATI.\r");
        pengontrol_pompa.matikan();
    }

    delay.delay_millis(2000); 
}

#[entry]
fn main() -> ! {
    // Inisialisasi perangkat keras modern esp-hal
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut delay = Delay::new();

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    
    // Konfigurasi pin 4 sebagai output dengan status awal LOW (mati)
    let pin_fisik_4 = Output::new(io.pins.gpio4, Level::Low);
    let mut pompa = PengontrolPompa::new(pin_fisik_4);

    // Kita set pH 4.0 agar masuk kondisi Bahaya dan Relay menyala
    let data = DataHidroponik::new(30.5, 60.5, 3.6, StatusPompa::Mati);

    // TAMBAHKAN INI: Jeda 1 detik agar terminal Wokwi siap menerima teks
    delay.delay_millis(5000); 
    esp_println::logger::init_logger_from_env();

    
    loop {
        loop_hidroponik(&data, &mut pompa, &mut delay);
    }
}
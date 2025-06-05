use std::{thread::sleep, time::Duration};

use embedded_dht_rs::dht22::Dht22;
use esp_idf_svc::hal::{delay::Delay, gpio::PinDriver, prelude::Peripherals};
use log::{info, warn};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Hello, world!");
}

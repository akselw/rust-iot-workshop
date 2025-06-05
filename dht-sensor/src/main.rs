mod sensor;

use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::peripherals::Peripherals;
use std::thread::sleep;
use std::time::Duration;
use anyhow::bail;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use crate::sensor::SmartSensor;
use log::info;
use dht_sensor::config::APP_CONFIG;
use dht_sensor::wifi::connect;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;

    let sysloop = EspSystemEventLoop::take()?;

    println!("{:?}: {:?}", APP_CONFIG.wifi_ssid, APP_CONFIG.wifi_ssid);

    let _wifi = match connect(
        APP_CONFIG.wifi_ssid,
        APP_CONFIG.wifi_pwd,
        peripherals.modem,
        sysloop,
    ) {
        Ok(inner) => {
            println!("Connected to Wi-Fi network!");
            inner
        }
        Err(err) => {
            // Red!
            bail!("Could not connect to Wi-Fi network: {:?}", err)
        }
    };





    let pin = PinDriver::input_output_od(peripherals.pins.gpio2).unwrap();

    let mut smart_sensor = SmartSensor::new(pin);

    smart_sensor.run()?;

    info!("Hello, world!");
    loop {
        sleep(Duration::from_secs(1));

        match smart_sensor.read() {
            Ok(sensor_reading) => log::info!(
                "DHT 22 Sensor - Temperature: {} °C, humidity: {} %",
                sensor_reading.temperature,
                sensor_reading.humidity
            ),
            Err(error) => log::error!("An error occurred while trying to read sensor: {:?}", error),
        }
    }

}

use embedded_dht_rs::dht22::Dht22;
use embedded_dht_rs::{SensorError, SensorReading};
use embedded_hal::digital::{InputPin, OutputPin};
use esp_idf_svc::hal::delay::Delay;

pub struct SmartSensor<P: InputPin + OutputPin> {
    dht22: Dht22<P, Delay>
}

impl<P : InputPin + OutputPin> SmartSensor<P> {
    pub fn new(pin: P) -> Self {
        Self { dht22 : Dht22::new(pin, Delay::default())}
    }

    pub fn read(&mut self) -> Result<SensorReading<f32>, SensorError> {
        self.dht22.read()
    }
}

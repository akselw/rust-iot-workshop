use std::thread::sleep;
use std::time::Duration;
use embedded_dht_rs::dht22::Dht22;
use embedded_dht_rs::{SensorError, SensorReading};
use embedded_hal::digital::{InputPin, OutputPin};
use esp_idf_svc::hal::delay::Delay;
use esp_idf_svc::http::Method;
use esp_idf_svc::http::server::{Configuration, EspHttpServer};
use esp_idf_svc::io::{EspIOError, Write};

pub struct SmartSensor<'a, P: InputPin + OutputPin> {
    dht22: Dht22<P, Delay>,
    server: Option<EspHttpServer<'a>>,
}

impl<'a, P: InputPin + OutputPin> SmartSensor<'a, P> {
    pub fn new(pin: P) -> Self {
        Self { dht22: Dht22::new(pin, Delay::default()), server: None }
    }

    pub fn read(&mut self) -> Result<SensorReading<f32>, SensorError> {
        self.dht22.read()
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        let conf = Configuration {
            http_port: 8080,
            ..Default::default()
        };

        let mut server = EspHttpServer::new(&conf)?;
        server.fn_handler("/alive", Method::Get,
                          |request| -> Result<(), EspIOError> {
                              let mut response = request.into_ok_response()?;
                              let res_text = "alive";
                              response.write_all(res_text.as_bytes())?;
                              Ok(())
                          }, )?;

        self.server = Some(server);

        println!("running server");
        Ok(())


    }
}

pub struct Config {
    pub wifi_ssid: &'static str,
    pub wifi_pwd: &'static str,
}

pub const APP_CONFIG: Config = Config {
    wifi_ssid: "Arve TLFsen",
    wifi_pwd: "ethernot",
};

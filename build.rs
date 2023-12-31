#[derive(Debug)]
#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    mqtt_host: &'static str,
    #[default("")]
    mqtt_user: &'static str,
    #[default("")]
    mqtt_pass: &'static str,
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Necessary because of this issue: https://github.com/rust-lang/cargo/issues/9641
    embuild::build::CfgArgs::output_propagated("ESP_IDF")?;
    embuild::build::LinkArgs::output_propagated("ESP_IDF")?;

    if !std::path::Path::new("cfg.toml").exists() {
        panic!("You need to create a `cfg.toml` file with your Wi-Fi credentials! Use `cfg.toml.example` for a template.");
    }
    // The constant `CONFIG` is auto-generated by `toml_config`.
    let app_config: Config = CONFIG;
    if app_config.wifi_ssid == "" || app_config.wifi_psk == "" {
        panic!("You need to set the Wi-Fi credentials in `cfg.toml`!");
    }
    if app_config.mqtt_host == "" {
        panic!("You need to set the MQTT credentials in `cfg.toml`!");
    }

    Ok(())
}

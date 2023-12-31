use esp_idf_hal::spi::SpiDriverConfig;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::timer::EspTaskTimerService;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;
use splitflap_rs::wifi;

use esp_idf_hal::{delay, gpio, prelude::*, spi};

use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};
use esp_idf_svc::mqtt::client::{EspMqttClient, EspMqttMessage, MqttClientConfiguration};
use embedded_svc::mqtt::client::{Event, QoS};
use esp_idf_svc::{wifi::AsyncWifi, wifi::EspWifi};
use futures::executor::block_on;
use mipidsi::Builder;
use std::thread;
use std::time::Duration;
use anyhow::Result;

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

const DEVICE_UID: &'static str = "splitflap_rs_device0";

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let app_config = CONFIG;

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take().expect("SysEventLoop should exist");
    let timer_service = EspTaskTimerService::new()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut wifi = AsyncWifi::wrap(
        EspWifi::new(peripherals.modem, sysloop.clone(), Some(nvs))?,
        sysloop,
        timer_service.clone(),
    )
    .expect("Failed to setup AsyncWifi");

    block_on(wifi::connect_wifi(
        app_config.wifi_ssid,
        app_config.wifi_psk,
        &mut wifi,
    ))?;

    info!("Startup Display!");

    let mosi = peripherals.pins.gpio19;
    let sclk = peripherals.pins.gpio18;
    let cs = peripherals.pins.gpio5;
    let dc = gpio::PinDriver::output(peripherals.pins.gpio16)?;
    let rst = gpio::PinDriver::output(peripherals.pins.gpio23)?;
    let mut backlight = gpio::PinDriver::output(peripherals.pins.gpio4)?;

    let spi_config = spi::config::Config::new()
        .baudrate(26.MHz().into())
        .data_mode(embedded_hal::spi::MODE_3);

    let spi = spi::SpiDeviceDriver::new_single(
        peripherals.spi2,
        sclk,
        mosi,
        Option::<gpio::Gpio0>::None,
        Some(cs),
        &SpiDriverConfig::new(),
        &spi_config,
    )?;

    let mut delay = delay::Ets;

    let display_interface = SPIInterfaceNoCS::new(spi, dc);

    let mut display = Builder::st7789_pico1(display_interface)
        .init(&mut delay, Some(rst))
        .unwrap();

    backlight.set_high()?;
    display.clear(Rgb565::BLACK).unwrap();
    log::info!("ST7789 initialized");

    let style: MonoTextStyle<'_, Rgb565> = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
    Text::new("startup mqtt", Point::new(20, 30), style)
        .draw(&mut display)
        .unwrap();
    
    let mut mqtt_client = start_mqtt_listener(&app_config)?;
    
    mqtt_client.subscribe(&format!("{}/text", DEVICE_UID), QoS::AtLeastOnce)?;
    
    let mut ii: u8 = 0;
    loop {
        thread::sleep(Duration::from_millis(1000));
        let counter_str = ii.to_string();
        let payload: &[u8] = counter_str.as_bytes();
        mqtt_client.publish(&format!("{}/counter", DEVICE_UID), QoS::AtLeastOnce, true, payload)?;

        ii = ii + 1;
    }
}

fn start_mqtt_listener(cfg: &Config) -> anyhow::Result<EspMqttClient> {
    // client_id needs to be unique
    let mqtt_conf = MqttClientConfiguration {
        client_id: Some("splitflap_display"),
        keep_alive_interval: Some(Duration::from_secs(120)),
        ..Default::default()
    };

    return Ok(EspMqttClient::new(
        format!("mqtt://{}:{}@{}", cfg.mqtt_user, cfg.mqtt_pass, cfg.mqtt_host),
        &mqtt_conf,
        move |msg_event| match msg_event {
            Ok(Event::Received(msg)) => process_mqtt_msg(msg),
            _ => log::warn!("Received non-OK from mqtt: {:?}", msg_event),
        },
    )?);
}

fn process_mqtt_msg(msg : &EspMqttMessage) {
    log::info!("Message from MQTT:: {:?}", msg);
}

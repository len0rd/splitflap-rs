pub mod wifi {

    use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
    use esp_idf_svc::{wifi::AsyncWifi, wifi::EspWifi};
    use log::*;

    pub async fn connect_wifi(
        ssid: &str,
        pass: &str,
        wifi: &mut AsyncWifi<EspWifi<'static>>,
    ) -> anyhow::Result<()> {
        let mut auth_method = AuthMethod::WPA2Personal;
        if ssid.is_empty() {
            error!("Missing WiFi name");
        }
        if pass.is_empty() {
            auth_method = AuthMethod::None;
            info!("Wifi password is empty");
        }
        let wifi_config = Configuration::Client(ClientConfiguration {
            ssid: ssid.into(),
            bssid: None,
            auth_method: auth_method,
            password: pass.into(),
            channel: None,
        });

        wifi.set_configuration(&wifi_config)
            .expect("Failed to set wifi configuration");

        info!("Starting wifi...");

        wifi.start().await?;

        info!("Connecting wifi...");

        wifi.connect().await?;

        info!("Waiting for DHCP lease...");

        wifi.wait_netif_up().await?;

        let ip_info = wifi.wifi().sta_netif().get_ip_info()?;

        info!("Wifi DHCP info: {:?}", ip_info);

        Ok(())
    }
}

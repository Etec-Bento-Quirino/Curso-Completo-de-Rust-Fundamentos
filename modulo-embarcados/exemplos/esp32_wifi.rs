use esp_idf_hal::prelude::*;
use esp_idf_sys as _;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::*;
use esp_idf_svc::http::server::*;
use std::sync::Arc;
use std::thread;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    
    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;
    
    let mut wifi = EspWifi::new(peripherals.modem, sys_loop, Some(nvs))?;
    
    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: "SEU_WIFI".into(),
        password: "SUA_SENHA".into(),
        ..Default::default()
    }))?;
    
    wifi.start()?;
    wifi.connect()?;
    
    while !wifi.is_connected()? {
        thread::sleep_ms(100);
    }
    
    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;
    println!("IP: {:?}", ip_info);
    
    let mut server = EspHttpServer::new(&Default::default())?;
    
    server.fn_handler("/", Method::Get, |req| {
        Ok(Response::new(200, "text/html", "Hello from ESP32!".as_bytes()))
    })?;
    
    server.fn_handler("/led", Method::Post, |req| {
        let body = req.into_body();
        println!("Received: {:?}", body);
        Ok(Response::new(200, "text/plain", "LED toggled!".as_bytes()))
    })?;
    
    println!("Server running on http://{:?}", ip_info.ip);
    
    loop {
        thread::sleep_ms(1000);
    }
}

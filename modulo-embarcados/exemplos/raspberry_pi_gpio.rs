use rppal::gpio::{Gpio, Level, Mode};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gpio = Gpio::new()?;
    
    // LED no pino 18
    let mut led = gpio.get(18)?.into_output();
    
    // Botão no pino 24
    let button = gpio.get(24)?.into_input_pullup();
    
    // Sensor de temperatura DHT22 no pino 4
    let dht_pin = gpio.get(4)?;
    
    println!("Sistema iniciado!");
    println!("Pressione o botão para alternar o LED");
    
    let mut led_state = false;
    
    loop {
        // Verificar botão
        if button.read() == Level::Low {
            led_state = !led_state;
            led.write(if led_state { Level::High } else { Level::Low });
            println!("LED: {}", if led_state { "LIGADO" } else { "DESLIGADO" });
            
            // Debounce
            thread::sleep(Duration::from_millis(200));
        }
        
        // Ler sensor de temperatura (simulado)
        let temperature = read_temperature(&dht_pin)?;
        println!("Temperatura: {:.1}°C", temperature);
        
        thread::sleep(Duration::from_secs(1));
    }
}

fn read_temperature(_pin: &rppal::gpio::IoPin) -> Result<f32, Box<dyn std::error::Error>> {
    // Simulação de leitura de temperatura
    Ok(22.5 + (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs() % 10) as f32)
}

extern crate embedded_graphics;
extern crate linux_embedded_hal as hal;
extern crate log;
extern crate pretty_env_logger;
extern crate ssd1306;
extern crate systemstat;

mod info;
mod monitor;

use log::{debug, info};

use std::env;

use monitor::Monitor;

const I2C_DEVICE: &'static str = "/dev/i2c-1";
const SSD1306_ADDR: u8 = 0x3C;

fn main() {
    if env::var("JETSON_MONITOR_LOG").is_err() {
        env::set_var("JETSON_MONITOR_LOG", "info");
    }
    if let Err(e) = pretty_env_logger::try_init_custom_env("JETSON_MONITOR_LOG") {
        panic!("Failed to initialize logger: {}", e);
    }
    let mut monitor = Monitor::new(SSD1306_ADDR, I2C_DEVICE);
    info!("Display initialized, entering loop...");
    loop {
        debug!("Updating information...");
        monitor.update();
        debug!("Information updated");
    }
}

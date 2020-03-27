use hal::I2cdev;

use ssd1306::interface::i2c::I2cInterface;
use ssd1306::mode::GraphicsMode;
use ssd1306::prelude::*;
use ssd1306::Builder;

type Ssd1306Display = GraphicsMode<I2cInterface<I2cdev>>;

pub struct DisplayWrapper {
    pub inner: Ssd1306Display,
}

impl DisplayWrapper {
    pub fn new(i2c_address: u8, i2c_dev_location: &'static str) -> Self {
        let i2c_dev = I2cdev::new(i2c_dev_location).expect("Failed to open I2C device");
        let mut display: Ssd1306Display = Builder::new()
            .with_i2c_addr(i2c_address)
            .size(DisplaySize::Display128x64)
            .connect_i2c(i2c_dev)
            .into();
        display.init().expect("Failed to initiate the dispay");
        Self { inner: display }
    }
}

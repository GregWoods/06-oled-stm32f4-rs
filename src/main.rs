#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use core::fmt::Write;
use ssd1306::{prelude::*, Builder as SSD1306Builder};
use stm32f4xx_hal::{
    prelude::*,
    i2c::I2c,
    stm32,
};


#[entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = stm32::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    // Set up I2C - SCL is PB8 and SDA is PB9; they are set to Alternate Function 4, open drain
    let gpiob = dp.GPIOB.split();
    let scl = gpiob.pb8.into_alternate_af4().set_open_drain();
    let sda = gpiob.pb9.into_alternate_af4().set_open_drain();
    let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks);

    // Set up the display: using terminal mode with 128x32 display
    let mut disp: TerminalMode<_> = SSD1306Builder::new().size(DisplaySize::Display128x64).connect_i2c(i2c).into();
    disp.init().unwrap();
    disp.clear().unwrap();
   
    //disp.write_str(buffer.as_str()).unwrap();
    disp.write_str("I Love Ali").unwrap();          
    
    loop {}
}

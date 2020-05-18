//! Blinks an LED
//!
//! This assumes that a LED is connected to pc13 as is the case on the blue pill board.
//!
//! Note: Without additional hardware, PC13 should not be used to drive an LED, see page 5.1.2 of
//! the reference manual for an explanation. This is not an issue on the blue pill.

#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;


use stm32f4xx_hal::{
    prelude::*,
    stm32,
    serial::{Serial, config::Config},
};
//use cortex_m_semihosting::hprintln;
use cortex_m_rt::entry;


#[entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = stm32::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    let gpioa = dp.GPIOA.split();

    let txpin = gpioa.pa9.into_alternate_af7();
    let rxpin = gpioa.pa10.into_alternate_af7();

    let mut serial1 = Serial::usart1(
        dp.USART1, 
        (txpin, rxpin), 
        Config::default(), 
        clocks).expect("error initialising USART1");

    //convert this into a function when I figure out how! Forcnow, inline it.
    //  and add it as write_str() into stm32f4xx-hal ... in the distant future
    for byte in b"Hello, welcome from Greg\n".iter() {
        while serial1.write(*byte).is_err() {}
    }

    

    loop {
        // Wait for reception of a single byte
        let received = nb::block!(serial1.read()).unwrap();

        // Send back previously received byte and wait for completion
        nb::block!(serial1.write(received)).ok();
    }
}

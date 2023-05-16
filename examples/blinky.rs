//! Turn an LED on/off depending on the state of a button
//!
//! This assumes that a LED is connected to GPIO4.
//! Additionally this assumes a button connected to GPIO9.
//! On an ESP32C3 development board this is the BOOT button.
//!
//! Depending on your target and the board you are using you should change the pins.
//! If your board doesn't have on-board LEDs don't forget to add an appropriate resistor.

use esp_idf_hal::delay::Delay;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio2)?;
    let mut i = 0;

    loop {
        // we are using thread::sleep here to make sure the watchdog isn't triggered
        Delay::delay_ms(1000);

        if i % 2 == 0 {
            led.set_high()?;
        } else {
            led.set_low()?;
        }
        i += 1;
    }
}

use esp_idf_hal::ledc::config::TimerConfig;
use esp_idf_hal::ledc::{LedcDriver, LedcTimerDriver};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use esp_idf_hal::prelude::*;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let timer_driver = LedcTimerDriver::new(
        peripherals.ledc.timer0,
        &TimerConfig::default().frequency(25.kHz().into()),
    )
    .unwrap();

    let mut driver = LedcDriver::new(
        peripherals.ledc.channel0,
        &timer_driver,
        peripherals.pins.gpio2,
    )
    .unwrap();

    let mut driver2 = LedcDriver::new(
        peripherals.ledc.channel1,
        &timer_driver,
        peripherals.pins.gpio3,
    )
    .unwrap();

    let mut driver3 = LedcDriver::new(
        peripherals.ledc.channel2,
        &timer_driver,
        peripherals.pins.gpio4,
    )
    .unwrap();

    for numerator in (0..255).cycle() {
        driver.set_duty(numerator).unwrap();
        driver2.set_duty(numerator).unwrap();
        driver3.set_duty(numerator).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

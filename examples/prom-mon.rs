#![no_std]
#![no_main]

use esp32_phantom::{clocks, hal, rtc, system, timer};
use esp_wifi::initialize_prom;
use hal::clock::{ClockControl, CpuClock};
use hal::Rng;
use hal::{peripherals::Peripherals, prelude::*, Rtc};

use esp32_hal::entry;
use esp_backtrace as _;

use esp_println::logger::init_logger;
use esp_println::println;

use esp_wifi::wifi::WifiError;

use num_traits::FromPrimitive;

#[macro_export]
macro_rules! esp_wifi_result {
    ($value:expr) => {
        if $value != esp_wifi::binary::include::ESP_OK as i32 {
            Err(WifiError::InternalError(
                FromPrimitive::from_i32($value).unwrap(),
            ))
        } else {
            core::result::Result::<(), WifiError>::Ok(())
        }
    };
}

use esp_wifi::binary::include::esp_wifi_start;

#[entry]
fn main() -> ! {
    init_logger(log::LevelFilter::Info);

    let peripherals = Peripherals::take();

    let system = system!(peripherals);
    let mut peripheral_clock_control = system.peripheral_clock_control;
    let clocks = clocks!(system);
    rtc!(peripherals);

    let timer = timer!(peripherals, clocks, peripheral_clock_control);
    initialize_prom(
        timer,
        Rng::new(peripherals.RNG),
        system.radio_clock_control,
        &clocks,
    )
    .unwrap();

    unsafe {
        esp_wifi_result!(esp_wifi_start()).unwrap();
    }

    println!("WiFi started!");
    loop {}
}

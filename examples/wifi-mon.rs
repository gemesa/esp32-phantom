#![no_std]
#![no_main]

use esp32_phantom::{clocks, get_wifi, hal, rtc, system, timer};

use embedded_svc::wifi::{AccessPointInfo, ClientConfiguration, Configuration, Wifi};

use esp_backtrace as _;
use esp_println::logger::init_logger;
use esp_println::println;
use esp_wifi::initialize;
use esp_wifi::wifi::utils::create_network_interface;
use esp_wifi::wifi::{WifiError, WifiMode};
use hal::clock::{ClockControl, CpuClock};
use hal::Rng;
use hal::{peripherals::Peripherals, prelude::*, Rtc};

use core::fmt::Write;
use heapless::String;

use smoltcp::iface::SocketStorage;

#[entry]
fn main() -> ! {
    init_logger(log::LevelFilter::Info);

    let peripherals = Peripherals::take();

    let system = system!(peripherals);
    let mut peripheral_clock_control = system.peripheral_clock_control;
    let clocks = clocks!(system);
    rtc!(peripherals);

    let timer = timer!(peripherals, clocks, peripheral_clock_control);
    initialize(
        timer,
        Rng::new(peripherals.RNG),
        system.radio_clock_control,
        &clocks,
    )
    .unwrap();

    let wifi = get_wifi!(peripherals);
    let mut socket_set_entries: [SocketStorage; 3] = Default::default();
    let (_iface, _device, mut controller, _sockets) =
        create_network_interface(wifi, WifiMode::Sta, &mut socket_set_entries);

    let client_config = Configuration::Client(ClientConfiguration {
        ..Default::default()
    });
    let res = controller.set_configuration(&client_config);
    println!("\rConfiguration status: {:?}", res);

    controller.start().unwrap();
    println!("\rController status: {:?}", controller.is_started());

    println!("\rAPs:");
    let res: Result<(heapless::Vec<AccessPointInfo, 10>, usize), WifiError> = controller.scan_n();
    if let Ok((res, _count)) = res {
        for ap in res {
            let mut bssid: String<12> = String::new();
            for byte in ap.bssid.iter() {
                let _ = write!(bssid, "{:02X}", byte);
            }
            println!(
                "\r{} | {} | {} | {} | {:?}",
                ap.ssid, bssid, ap.channel, ap.signal_strength, ap.auth_method
            );
        }
    }

    loop {}
}

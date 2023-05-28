# esp32-phantom

esp32-phantom is a Rust sandbox to explore the WiFi and BLE capabilities of the ESP32, for example:
- WiFi monitoring
- promiscuous mode
- pcap logging
- crypto price monitoring

If you are serious about security analysis/pen testing I suggest to use an Alfa adapter instead such as [AWUS036AXML](https://alfa-network.eu/alfa-usb-adapter-awus036axml) or [AWUS036ACHM](https://alfa-network.eu/awus036achm) in combination with [hcxdumptool](https://github.com/ZerBea/hcxdumptool). Both of them are supported with excellent in-kernel drivers. 

## Toolchain installation and firmware building

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Rust for Xtensa](https://esp-rs.github.io/book/installation/index.html)

TLDR installation steps:

```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ cargo install espflash
$ cargo install espup
$ espup install
$ . $HOME/export-esp.sh
$ cd <your-esp32-phantom-repo>
$ rustup override set esp
```
### How to build


```
$ cargo build --release --examples --features "esp32,wifi"
```
## Examples

### Blinky

```
$ espflash /dev/ttyUSB0 target/xtensa-esp32-none-elf/release/examples/blinky
```

### WiFi monitoring

```
$ espflash /dev/ttyUSB0 target/xtensa-esp32-none-elf/release/examples/wifi-mon
$ screen /dev/ttyUSB0 115200
$ # press EN/RST button
ets Jun  8 2016 00:22:57

rst:0x1 (POWERON_RESET),boot:0x13 (SPI_FAST_FLASH_BOOT)
configsip: 0, SPIWP:0xee
clk_drv:0x00,q_drv:0x00,d_drv:0x00,cs0_drv:0x00,hd_drv:0x00,wp_drv:0x00
mode:DIO, clock div:2
load:0x3fff0048,len:12
ho 0 tail 12 room 4
load:0x3fff0054,len:4800
load:0x40078000,len:17448
load:0x4007c428,len:4840
entry 0x4007c6a0
Configuration status: Ok(())
Controller status: Ok(true)
APs:
xxxxx | XXXXXXXXXXXX | 3 | -32 | WPA2Personal
xxxxx | XXXXXXXXXXXX | 7 | -79 | WPA2Personal
xxxxx | XXXXXXXXXXXX | 7 | -79 | WPA2Personal
xxxxx | XXXXXXXXXXXX | 11 | -86 | WPA2Personal
```

### Promiscuous mode

This mode can not be activated with the current latest version of esp-wifi because `queue_msg_waiting()` is not implemented yet and this function is mandatory for promiscuous mode:

```
cat esp-wifi/src/wifi/os_adapter.rs
...
pub unsafe extern "C" fn queue_msg_waiting(_queue: *mut crate::binary::c_types::c_void) -> u32 {
    todo!("queue_msg_waiting")
}
...
```

```
$ espflash /dev/ttyUSB0 target/xtensa-esp32-none-elf/release/examples/prom-mon
$ screen /dev/ttyUSB0 115200
$ # press EN/RST button
ets Jun  8 2016 00:22:57

rst:0x1 (POWERON_RESET),boot:0x13 (SPI_FAST_FLASH_BOOT)
configsip: 0, SPIWP:0xee
clk_drv:0x00,q_drv:0x00,d_drv:0x00,cs0_drv:0x00,hd_drv:0x00,wp_drv:0x00
mode:DIO, clock div:2
load:0x3fff0048,len:12
ho 0 tail 12 room 4
load:0x3fff0054,len:4800
load:0x40078000,len:17448
load:0x4007c428,len:4840
entry 0x4007c6a0
WiFi started!
              
               
               !! A panic occured in 'esp-wifi/esp-wifi/src/wifi/os_adapter.rs', at line 599, column 5
                                                                                                       
                                                                                                       PanicInfo {
                                                                                                                      payload: Any { .. },
                        message: Some(
                                              not yet implemented: queue_msg_waiting,
                                                                                         ),
                                                                                               location: Location {
                                                                                                                           file: "esp-wifi/esp-wifi/src/wifi/os_adapter.rs",
                                                              line: 599,
                                                                                col: 5,
                                                                                           },
                                                                                                 can_unwind: true,
                                                                                                                  }
                                                                                                                    
                                                                                                                    Backtrace:
         
         0x4008afa8
                   0x401045e4
                             0x400870e0
                                       0x40087312
                                                 0x40083d80
                                                           0x400d8e03
                                                                     0x400d5e04
                                                                               0x40000000
```

The following functions have been added to the esp-wifi submodule to test promiscuous mode:

- `initialize_prom()` (initialize internals + call `wifi_init_prom()`)
- `wifi_init_prom()` (initialize WiFi)
- `recv_cb_prom()` (callback function, called when a packet is received)

These changes can not be committed to the submodule directly so the patch file `esp-wifi-promiscuous.patch` has been created. How to apply and build:

```
$ cd esp-wifi
$ git apply ../esp-wifi-promiscuous.patch
$ cd ..
$ cargo build --release --examples --features "esp32,wifi"
```

[prom-mon.c](https://www.hackster.io/p99will/esp32-wifi-mac-scanner-sniffer-promiscuous-4c12f4) has been used as a reference to set promiscuous mode.

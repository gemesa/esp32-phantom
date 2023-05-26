# esp32-phantom

:warning: Work in progress! :warning:

esp32-phantom is a Rust sandbox to explore the Wi-Fi and BLE capabilities of the ESP32, for example:
- Wi-Fi monitoring
- promiscuous mode
- pcap logging
- crypto price monitoring

If you are serious about security analysis/pen testing I suggest to use an Alfa adapter instead such as [AWUS036AXML](https://alfa-network.eu/alfa-usb-adapter-awus036axml) or [AWUS036ACHM](https://alfa-network.eu/awus036achm) in combination with [hcxdumptool](https://github.com/ZerBea/hcxdumptool). Both of them are supported with excellent in-kernel drivers. 

## Toolchain installation and firmware building

### Prerequisites

The following tools are necessary for building:

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

Invoke the following command:

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
AccessPointInfo { ssid: "xxx", bssid: [xx, xx, xx, xx, xx, xx], channel: 3, secondary_channel: None, signal_strength: -31, protocols: EnumSet(), auth_method: WPA2Personal }
AccessPointInfo { ssid: "xxx", bssid: [xx, xx, xx, xx, xx, xx], channel: 7, secondary_channel: Above, signal_strength: -82, protocols: EnumSet(), auth_method: WPA2Personal }
AccessPointInfo { ssid: "xxx", bssid: [xx, xx, xx, xx, xx, xx], channel: 7, secondary_channel: Above, signal_strength: -83, protocols: EnumSet(), auth_method: WPA2Personal }
AccessPointInfo { ssid: "xxx", bssid: [xx, xx, xx, xx, xx, xx], channel: 1, secondary_channel: None, signal_strength: -89, protocols: EnumSet(), auth_method: WPA2Personal }
AccessPointInfo { ssid: "xxx", bssid: [xx, xx, xx, xx, xx, xx], channel: 1, secondary_channel: Above, signal_strength: -89, protocols: EnumSet(), auth_method: WPAWPA2Personal }
AccessPointInfo { ssid: "xxx", bssid: [xx, xx, xx, xx, xx, xx], channel: 1, secondary_channel: Above, signal_strength: -90, protocols: EnumSet(), auth_method: WPA2Personal }
```

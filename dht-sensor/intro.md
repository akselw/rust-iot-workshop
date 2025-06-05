---
title: "Rust IoT workshop"
authors:
  - Isak Sunde Singh
  - Fredrik Mørstad
options:
  incremental_lists: true
---

# Agenda

- Short intro to embedded programming
- Workshop with ESP32, in pairs

---

# Embedded programming

Wide range of meanings, from the smallest of microchips with a few kB of RAM, to Raspberry Pi's with up to 16 GB RAM.

Typically split in two:

## Hosted environments

Similar to a regular environment. Some system interface, libraries, networking, file systems, etc. Perhaps you also have access to certain I/O devices you normally don't.

## Bare metal environments

Cannot load a standard library, no preloaded stuff, no system support for networking, file systems, etc.
In Rust we need to use `#![no_std]` to disable the standard library.

---

# `#![no_std]`

`#[no_std]` is an attribute that disables the standard library, and since we want to disable it for the entire crate, we write `#![no_std]` (note the `!`).

---

# Espressif ESP32

- ESP32 is a family of microcontrollers with Wi-Fi and Bluetooth
- Low energy
- GPIO -> General purpose IO pins for... general purposes, such as controlling peripherals such as LEDs, thermometers, etc.
- Very cheap, ~5 USD per controller

---

# Hardware abstraction layers (HAL 🔴)

Even though many microcontrollers are quite difficult in capabilities, it is nice to not have to specialize implementations for every controller. It would be nice with an abstraction layer over e.g. pin support, drivers for communicating through I2C, etc.

```
Sweet Rust code without any hardware knowledge
                      HAL
                    HARDWARE
```

---

# HALs

Example usage of HALs and full-_ish_ Rust-code for a blinking LED with an ESP32

```rust +line_numbers {1-2,15|3-4|6-7|9-14|1-15}
#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Set GPIO0 as an output, and set its state high initially.
    let mut led = Output::new(peripherals.GPIO0, Level::High, OutputConfig::default());

    loop {
        led.toggle();
        // Wait for half a second
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }
}
```

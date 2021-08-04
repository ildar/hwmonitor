#![no_main]
#![no_std]

use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;
use nrf52832_hal as hal;
use nrf52832_hal::gpio::Level;
use rtt_target::{rprint, rprintln, rtt_init_print};

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    let p = hal::pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0);
    //let button = port0.p0_13.into_pullup_input();
    let mut led = port0.p0_24.into_push_pull_output(Level::Low);
    let mut on = true;

    rprintln!("HW monitor demo starting");
    loop {
        if on {
            led.set_high().unwrap();
        } else {
            led.set_low().unwrap();
        }
        on = ! on;
        rprint!(".");
        cortex_m::asm::delay(32_000_000);
    }
}

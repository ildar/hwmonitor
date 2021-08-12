#![no_main]
#![no_std]

use embedded_hal::digital::v2::OutputPin;
use nrf52832_hal as hal;
use nrf52832_hal::gpio::Level;
use rtt_target::{rprint, rprintln, rtt_init, set_print_channel};

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let rtt_channels = rtt_init! {
            up: {
                0: {
                    size: 1024
                    name: "Terminal"
                }
            }
            down: {
                0: {
                    size: 32
                    mode: BlockIfFull
                    name: "Terminal"
                }
            }
        };
    set_print_channel(rtt_channels.up.0);
    let p = hal::pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0);
    let mut led = port0.p0_24.into_push_pull_output(Level::Low);
    let mut on = true;
    let mut buf = [0u8; 255];
    let mut cmd = rtt_channels.down.0;

    initial_diags();
    loop {
        if on {
            led.set_high().unwrap();
        } else {
            led.set_low().unwrap();
        }
        on = ! on;
        let bufn = cmd.read(&mut buf);
        if bufn > 0 {
            rprint!("{}", buf[0] as char);
        } else {
            rprint!(".");
            cortex_m::asm::delay(32_000_000);
        };
    }
}

fn initial_diags() {
    // RTT working? just print and see!
    rprintln!("HW monitor demo starting");
}

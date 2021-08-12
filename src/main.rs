#![no_main]
#![no_std]

use nrf52832_hal as _;
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
    let mut buf = [0u8; 255];
    let mut cmd_channel = rtt_channels.down.0;

    initial_diags();
    loop {
        let bufn = cmd_channel.read(&mut buf);
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

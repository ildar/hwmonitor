#![no_main]
#![no_std]

use nrf52832_hal as _;
use rtt_target::{rprintln, rtt_init, set_print_channel};

mod root;
use root::*;

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

    let mut buf_pos = 0;
    let mut buf = [0u8; 255];
    let mut cmd_channel = rtt_channels.down.0;

    initial_diags();
    let mut exec_context = ExecContext::new();
    exec_context.print_menu();
    loop {
        let bufn = cmd_channel.read(&mut buf[buf_pos..]);
        buf_pos += bufn;
        let mut cmd_end = 0;
        for (i, &c) in buf[..buf_pos].iter().enumerate() {
            if c == b'\n' { cmd_end = i; break; };
        }
        if cmd_end > 0 {
            //exec_context.execute(&buf[..cmd_end]);
            buf_pos = 0; //FIXME: dropping the rest of the RTT input
        };
        if bufn == 0 {
            exec_context.idle();
            cortex_m::asm::delay(32_000_000);
        };
    }
}

fn initial_diags() {
    // RTT working? just print and see!
    rprintln!("HW monitor demo starting");
}

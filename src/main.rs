#![no_main]
#![no_std]

use nrf52832_hal as hal;
use rtt_target::{rprintln, rprint, rtt_init, set_print_channel};
#[cfg(not(test))]
use panic_rtt_target as _;

mod root;
use root::*;

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
    // get some peripherals for use
    let periph = hal::pac::Peripherals::take().unwrap();
    exec_context.twim_hw.replace(Some(periph.TWIM0));

    // start interactive loop
    exec_context.print_menu();
    rprint!("\n> ");
    loop {
        let bufn = cmd_channel.read(&mut buf[buf_pos..]);
        buf_pos += bufn;
        let mut cmd_end = -1;
        for (i, &c) in buf[..buf_pos].iter().enumerate() {
            if c == b'\n' { cmd_end = i as i32; break; };
        }
        if cmd_end > 0 {
            // FIXME: rprint!(&buf[..cmd_end]); // echo
            exec_context.execute(&buf[..cmd_end as usize]);
            buf_pos = 0; //FIXME: dropping the rest of the RTT input
            rprint!("\n> ");
        } else if cmd_end == 0 { // bare '\n'
            buf_pos = 0;
            rprint!("\n> ");
        } else {
            exec_context.idle();
            cortex_m::asm::delay(1_000_000);
        };
    }
}

fn initial_diags() {
    // RTT working? just print and see!
    rprintln!("HW monitor demo starting");
}

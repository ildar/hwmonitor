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
    set_clocks();

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

    rprint!("Checking last reset reason.. ");
    let power_reg = hal::pac::POWER::ptr();
    let p = unsafe { (*power_reg).resetreas.read() };
    rprintln!("0x{:08x}", p.bits());

    rprint!("\n");
}

fn set_clocks() {
    rprint!("Checking system clock.. ");
    let clock_reg = hal::pac::CLOCK::ptr();
    let hfclkstat = unsafe { (*clock_reg).hfclkstat.read() };
    rprint!("hfclkstat=0x{:08x} ", hfclkstat.bits());
    let lfclkstat = unsafe { (*clock_reg).lfclkstat.read() };
    rprintln!("lfclkstat=0x{:08x} ", lfclkstat.bits());

    rprint!("Switching HF clock to external.. ");
    unsafe {
        (*clock_reg).tasks_hfclkstart.write(|w| w.bits(1));
    };
    cortex_m::asm::delay(1_000_000); // more then 0.36ms (per datasheet)
    rprintln!("done.");

    rprint!("Switching LF clock to external.. ");
    unsafe {
        (*clock_reg).tasks_lfclkstop.write(|w| w.bits(1));
        (*clock_reg).lfclksrc
            .write(move |w| w.src().xtal().bypass().bit(false).external().bit(true));
        (*clock_reg).tasks_lfclkstart.write(|w| w.bits(1));
    };
    cortex_m::asm::delay(36_000_000); // more then 0.25s (per datasheet)
    rprintln!("done.");

    if lfclkstat.state() == hal::pac::clock::lfclkstat::STATE_A::NOTRUNNING {
        rprint!("External LF clock failed, switching to internal.. ");
        unsafe {
            (*clock_reg).tasks_lfclkstop.write(|w| w.bits(1));
            (*clock_reg).lfclksrc
                .write(move |w| w.src().rc());
            (*clock_reg).tasks_lfclkstart.write(|w| w.bits(1));
        };
        cortex_m::asm::delay(36_000_000); // more then 0.25s (per datasheet)
        rprintln!("done.");
    }

    rprint!("Checking system clock (again).. ");
    let clock_reg = hal::pac::CLOCK::ptr();
    let hfclkstat = unsafe { (*clock_reg).hfclkstat.read() };
    rprint!("hfclkstat=0x{:08x} ", hfclkstat.bits());
    let lfclkstat = unsafe { (*clock_reg).lfclkstat.read() };
    rprintln!("lfclkstat=0x{:08x} ", lfclkstat.bits());
}

use nrf52832_hal as hal;
use hal::twim::*;
use hal::gpio::{ Pin, Disconnected };
use rtt_target::{rprint, rprintln};
use super::ExecContext;
use super::text_parser::*;

// i2c menu
pub fn print_menu() {
    rprint!(concat!(
        "i2c menu: \n",
        "  h or ?: \t print this menu prompt \n",
        "  r: \t return to main menu \n",
        "  S sclpin# sdapin#: \t initilize i2c with given pins and scan i2c bus \n",
        ""));
}

pub fn execute(s: &[u8], context: &mut ExecContext) {
    match s[0] {
        b'h' => print_menu(),
        b'?' => print_menu(),
        b'r' => {
                release_i2cs(context);
                context.set_handlers(super::print_menu, super::execute, super::idle);
            },
        b'S' => scan_i2c(s, context),
        _ => { rprintln!("unknown command"); print_menu(); },
    }
}

pub fn idle(_context: &ExecContext) {
}

fn scan_i2c(s: &[u8], _context: &mut ExecContext) {
    let mut args = [None; 2]; let mut __ = None;
    parse_command(s, &mut args, &mut __);
    if args[0] == None || args[0].unwrap() > 31 ||
       args[1] == None || args[1].unwrap() > 31 {
        rprintln!("Wrong pin numbers format");
        return;
    };
    let scl = unsafe { Pin::<Disconnected>::from_psel_bits(args[0].unwrap())}
        .into_floating_input();
    let sda = unsafe { Pin::<Disconnected>::from_psel_bits(args[1].unwrap())}
        .into_floating_input();
    let periph = hal::pac::Peripherals::take().unwrap();
    let mut bus = Twim::new(periph.TWIM0, Pins { scl, sda }, Frequency::K100);
    let mut empty_buf = [0_u8;0];
    rprintln!("Scanning the i2c bus");
    for i in 1..127 {
        match bus.write(i, &mut empty_buf[..]) {
            Ok(_) => rprintln!("  found slave at addr 0x{:02x}", i),
            _ => {},
        }
    }
}

fn release_i2cs(_context: &mut ExecContext) {
}


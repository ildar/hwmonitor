use core::cell::RefCell;
use nrf52832_hal as hal;
use rtt_target::{rprint, rprintln};
#[path = "text_parser.rs"]
mod text_parser;
use text_parser::*;
#[path = "gpio.rs"]
mod gpio;
#[path = "i2c.rs"]
mod i2c;

// Common for all submenus
pub struct ExecContext {
    print_menu_fn: fn(),
    execute_fn: fn(&[u8], &mut ExecContext),
    idle_fn: fn(& ExecContext),
    pub input_pin: RefCell<Option<
        hal::gpio::Pin <
            hal::gpio::Input<hal::gpio::Floating>
        >
    >>,
    pub out_pins: [Option<u32>;4],
}

impl ExecContext {
    pub fn new() -> ExecContext {
        ExecContext {
            print_menu_fn: print_menu,
            execute_fn: execute,
            idle_fn: idle,
            input_pin: RefCell::new(None),
            out_pins: [None;4],
        }
    }
    pub fn set_handlers(&mut self,
        print_menu_fn: fn(),
        execute_fn: fn(&[u8], &mut ExecContext),
        idle_fn: fn(&ExecContext) ) {
        self.print_menu_fn = print_menu_fn;
        self.execute_fn = execute_fn;
        self.idle_fn = idle_fn;
        self.print_menu();
    }

    pub fn print_menu(&self) {
        (self.print_menu_fn)()
    }
    pub fn execute(&mut self, s: &[u8]) {
        (self.execute_fn)(s, self)
    }
    pub fn idle(&self) {
        (self.idle_fn)(self)
    }
    pub fn release_out_pins(&mut self) {
        self.out_pins.iter_mut()
            .filter(|p| p.is_some())
            .for_each(|p| {
                unsafe { hal::gpio::Pin::<hal::gpio::Disconnected>
                    ::from_psel_bits(p.unwrap()) }.into_floating_input();
                *p = None;
            });
    }
}

// root menu
pub fn print_menu() {
    rprint!(concat!(
        "HW monitor, version 0.01 \n",
        "Main menu: \n",
        "  h or ?: \t print this menu prompt \n",
        "  r: \t return to main menu \n",
        "  px N @addr: \t hexdump of N bytes at(@) address \n",
        "    example: px 4 @0x72000 \n",
        "  g: \t GPIO \n",
        "  i: \t i2c \n",
        "  R: \t RESET MCU \n",
        ""));
}

pub fn execute(s: &[u8], context: &mut ExecContext) {
    match s[0] {
        b'h' => print_menu(),
        b'?' => print_menu(),
        b'r' => print_menu(),
        b'p' => hexdump(s),
        b'w' => write_mem(s),
        b'g' => context.set_handlers(gpio::print_menu, gpio::execute, gpio::idle),
        b'i' => context.set_handlers(i2c::print_menu, i2c::execute, i2c::idle),
        b'R' => reset_mcu(),
        _ => { rprintln!("unknown command"); print_menu(); },
    }
}

pub fn idle(_: & ExecContext) {
    //rprint!(".");
}

fn hexdump(s: &[u8]) {
    let mut args = [None; 1]; let mut addr = None;
    parse_command(s, &mut args, &mut addr);
    let count = args[0].unwrap_or(256);
    let start = addr.unwrap_or(0x0);
    for i in 0..count {
        if i%16 == 0 { rprint!("\n0x{:08x} ", start+i) };
        let val:u8 = unsafe {
            core::ptr::read((start + i) as *const u8)
        };
        rprint!("{:02x}", val);
        if i%2 == 1 { rprint!(" ") };
    };
}

//TODO:
fn write_mem(_s: &[u8]) {
}

fn reset_mcu() -> ! {
    rprintln!("\n\n Resetting MCU \n\n\n");
    hal::pac::SCB::sys_reset();
}

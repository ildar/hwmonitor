use nrf52832_hal as _;
use rtt_target::{rprint, rprintln};
#[path = "text_parser.rs"]
mod text_parser;
use text_parser::*;

// Common for all submenus
pub struct ExecContext {
    print_menu_fn: fn(),
    execute_fn: fn(&[u8], &mut ExecContext),
    idle_fn: fn(),
}

impl ExecContext {
    pub fn new() -> ExecContext {
        ExecContext {
            print_menu_fn: print_menu,
            execute_fn: execute,
            idle_fn: idle,
        }
    }

    pub fn print_menu(&self) {
        (self.print_menu_fn)();
    }
    pub fn execute(&mut self, s: &[u8]) {
        (self.execute_fn)(s, self);
    }
    pub fn idle(&mut self) {
        (self.idle_fn)();
    }
}

// root menu
pub fn print_menu() {
    rprint!(concat!(
        "HW monitor, version 0.01 \n",
        "Main menu: \n",
        "  h or ?: \tprint this menu prompt \n",
        "  r: \t return to main menu \n",
        "  px N @addr_in_hex: \t hexdump of N bytes at(@) address \n",
        "    example: px 4 @00072000 \n",
        "    WARNING: address is exactly 8 hex-digits! \n",
        "\n> "));
}

pub fn execute(s: &[u8], context: &mut ExecContext) {
    match s[0] {
        b'h' => print_menu(),
        b'?' => print_menu(),
        b'r' => print_menu(),
        b'p' => hexdump(s),
        b'w' => write_mem(s),
        _ => { rprintln!("unknown command"); print_menu(); },
    }
}

pub fn idle() {
    //rprint!(".");
}

fn hexdump(s: &[u8]) {
    let res = parse_px(s, (0,16));
    if res == None { rprintln!("Failed to parse `px` command"); return; }
    let (start, count) = res.unwrap();
    rprint!("0x{:08x} ", start);
    for i in 0..count {
        let val:u8 = unsafe {
            core::ptr::read((start + i) as *const u8)
        };
        rprint!("{:02x} ", val);
    };
    rprint!("\n> ");
}

fn write_mem(s: &[u8]) {
}

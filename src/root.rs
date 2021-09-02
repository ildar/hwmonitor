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
        "  px N @addr: \t hexdump of N bytes at(@) address \n",
        "    example: px 4 @0x72000 \n",
        "\n> "));
}

pub fn execute(s: &[u8], _context: &mut ExecContext) {
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
    let res = parse_px(s, (256,0x0));
    if res.is_err() { rprintln!("Failed to parse `px` command"); return; }
    let (count, start) = res.unwrap();
    for i in 0..count {
        if i%16 == 0 { rprint!("\n0x{:08x} ", start+i) };
        let val:u8 = unsafe {
            core::ptr::read((start + i) as *const u8)
        };
        rprint!("{:02x}", val);
        if i%2 == 1 { rprint!(" ") };
    };
    rprint!("\n> ");
}

fn write_mem(_s: &[u8]) {
}

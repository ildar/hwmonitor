use nrf52832_hal as _;
use rtt_target::{rprint, rprintln};
use super::ExecContext;
//use super::text_parser::*;

// GPIO menu
pub fn print_menu() {
    rprint!(concat!(
        "GPIO menu: \n",
        "  h or ?: \t print this menu prompt \n",
        "  r: \t return to main menu \n",
        "\n> "));
}

pub fn execute(s: &[u8], context: &mut ExecContext) {
    match s[0] {
        b'h' => print_menu(),
        b'?' => print_menu(),
        b'r' => context.set_handlers(super::print_menu, super::execute, super::idle),
        _ => { rprintln!("unknown command"); print_menu(); },
    }
}

pub fn idle() {
    rprint!(".");
}


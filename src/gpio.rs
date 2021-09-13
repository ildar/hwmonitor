use nrf52832_hal as hal;
use hal::gpio::*;
use hal::prelude::InputPin as _;
use rtt_target::{rprint, rprintln};
use super::ExecContext;
use super::text_parser::*;

// GPIO menu
pub fn print_menu() {
    rprint!(concat!(
        "GPIO menu: \n",
        "  h or ?: \t print this menu prompt \n",
        "  r: \t return to main menu \n",
        "  g gpio#: \t set GPIO pin to read",
        ""));
}

pub fn execute(s: &[u8], context: &mut ExecContext) {
    match s[0] {
        b'h' => print_menu(),
        b'?' => print_menu(),
        b'r' => context.set_handlers(super::print_menu, super::execute, super::idle),
        b'g' => read_gpio(s, context),
        _ => { rprintln!("unknown command"); print_menu(); },
    }
}

pub fn idle(context: &ExecContext) {
    let pin = context.input_pin.replace(None);
    if pin.is_none() { return; };
    let the_pin = pin.unwrap();
    rprintln!("GPIO: {}={}", the_pin.pin(), the_pin.is_high().unwrap());
    context.input_pin.replace(Some(the_pin));
}

fn read_gpio(s: &[u8], context: &mut ExecContext) {
    let _ = context.input_pin.replace(None); // discard this pin
    let (gpiono,) = parse_g(s);
    if gpiono == None || gpiono.unwrap() > 31 {
        return;
    }

    let pin = unsafe { Pin::<Disconnected>::from_psel_bits(gpiono.unwrap())}
        .into_floating_input();
    context.input_pin.replace(Some(pin));
}

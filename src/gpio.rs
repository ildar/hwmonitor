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
        "  g gpio#: \t set GPIO pin to read \n",
        "  G gpio# val: \t set pin high (1) or low (0) \n",
        ""));
}

pub fn execute(s: &[u8], context: &mut ExecContext) {
    match s[0] {
        b'h' => print_menu(),
        b'?' => print_menu(),
        b'r' => {
                release_gpios(context);
                context.set_handlers(super::print_menu, super::execute, super::idle);
            },
        b'g' => read_gpio(s, context),
        b'G' => write_gpio(s, context),
        b'#' => (),
        _ => { rprintln!("unknown command"); print_menu(); },
    }
}

pub fn idle(context: &ExecContext) {
    let pin = context.input_pin.borrow();
    let the_pin = if pin.is_none() { return; } else {
        pin.as_ref().unwrap()
    };
    rprintln!("GPIO: {}={}", the_pin.pin(), the_pin.is_high().unwrap());
}

fn read_gpio(s: &[u8], context: &mut ExecContext) {
    let pin = context.input_pin.replace(None);
    if pin.is_some() { pin.unwrap().into_disconnected(); }; // discard this pin
    let mut args = [None; 1]; let mut __ = None;
    parse_command(s, &mut args, &mut __);
    if args[0] == None || args[0].unwrap() > 31 {
        return;
    };

    let pin = unsafe { Pin::<Disconnected>::from_psel_bits(args[0].unwrap()) }
        .into_floating_input();
    context.input_pin.replace(Some(pin));
}

fn write_gpio(s: &[u8], context: &mut ExecContext) {
    context.release_out_pins();
    let mut args = [None; 2]; let mut __ = None;
    parse_command(s, &mut args, &mut __);
    if args[0] == None || args[0].unwrap() > 31 {
        return;
    };
    { // unset input pin if user demands to output it
        let pin = context.input_pin.borrow();
        let the_pin = if pin.is_none() { return; } else {
            pin.as_ref().unwrap()
        };
        if the_pin.pin() as u32 == args[0].unwrap() {
            let pin = context.input_pin.replace(None);
            if pin.is_some() { pin.unwrap().into_disconnected(); }; // discard this pin
        }
    };
    let state = if args[1].is_none() || args[1].unwrap() == 0 { Level::Low } else { Level::High };

    unsafe { Pin::<Disconnected>::from_psel_bits(args[0].unwrap())}
        .into_push_pull_output(state);
    context.out_pins[0] = args[0];
}

fn release_gpios(context: &mut ExecContext) {
    read_gpio(b"", context);
    context.release_out_pins();
}


use nrf52832_hal as _;
use rtt_target::{rprint, rprintln};

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
    rprint!(concat!("Main menu: \n",
        "  h or ?: \tprint this menu prompt \n",
        "  r: \t return to main menu\n",
        ));
}

pub fn execute(s: &[u8], context: &mut ExecContext) {
    match s[0] {
        b'h' => print_menu(),
        b'?' => print_menu(),
        b'r' => print_menu(),
        _ => { rprintln!("unknown command"); print_menu(); },
    }
}

pub fn idle() {
    rprint!(".");
}


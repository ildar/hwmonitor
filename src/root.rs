use nrf52832_hal as _;
use rtt_target::{rprint, rprintln};

// Common for all submenus
pub struct ExecContext {
    print_menu_fn: fn(),
    execute_fn: fn(),
    idle_fn: fn(),
}

impl ExecContext {
    pub fn new() -> ExecContext {
        ExecContext {
            print_menu_fn: print_menu,
            execute_fn: print_menu,
            idle_fn: print_menu,
        }
    }

    pub fn print_menu(&self) {
        (self.print_menu_fn)();
    }
    /* pub fn execute(&mut self, ..) {
        (self.execute_fn)(..);
    }
    */
    pub fn idle(&mut self) {
        (self.idle_fn)();
    }
}

// root menu
pub fn print_menu() {
    rprintln!("Main menu:");
}

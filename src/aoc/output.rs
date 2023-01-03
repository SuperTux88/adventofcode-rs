use std::sync::atomic::{AtomicBool, Ordering};

static OUTPUT: AtomicBool = AtomicBool::new(true);
static DEBUG: AtomicBool = AtomicBool::new(true);

/// Disables output to stdout
pub fn disable_output() {
    OUTPUT.store(false, Ordering::Relaxed);
    DEBUG.store(false, Ordering::Relaxed);
}

/// Disables debug output
pub fn disable_debug() {
    DEBUG.store(false, Ordering::Relaxed);
}

pub fn is_debug_enabled() -> bool {
    DEBUG.load(Ordering::Relaxed)
}

/// Prints a message with newline to stdout if output is enabled
pub fn println(msg: String) {
    if OUTPUT.load(Ordering::Relaxed) {
        println!("{}", msg);
    }
}

pub fn println_debug(msg: String) {
    if DEBUG.load(Ordering::Relaxed) {
        println!("│  {}", msg);
    }
}

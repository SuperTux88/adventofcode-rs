use std::sync::atomic::{AtomicBool, Ordering};

static OUTPUT: AtomicBool = AtomicBool::new(true);

/// Disables output to stdout for benchmark mode
pub fn disable_output() {
    OUTPUT.store(false, Ordering::Relaxed);
}

/// Prints a message to stdout if output is enabled
pub fn print(msg: String) {
    if OUTPUT.load(Ordering::Relaxed) {
        print!("{}", msg);
    }
}

/// Prints a message with newline to stdout if output is enabled
pub fn println(msg: String) {
    if OUTPUT.load(Ordering::Relaxed) {
        println!("{}", msg);
    }
}

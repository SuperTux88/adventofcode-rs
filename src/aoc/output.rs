use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

static OUTPUT: AtomicBool = AtomicBool::new(true);
static DEBUG: AtomicBool = AtomicBool::new(true);

#[cfg(feature = "wasm")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

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
        #[cfg(all(feature = "wasm", target_arch = "wasm32"))]
        log(&msg);

        #[cfg(not(all(feature = "wasm", target_arch = "wasm32")))]
        println!("{}", msg);
    }
}

pub fn println_debug(msg: String) {
    if DEBUG.load(Ordering::Relaxed) {
        #[cfg(all(feature = "wasm", target_arch = "wasm32"))]
        log(&msg);

        #[cfg(not(all(feature = "wasm", target_arch = "wasm32")))]
        println!("│  {}", msg);
    }
}

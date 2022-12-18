static mut OUTPUT: bool = true;

/// Disables output to stdout for benchmark mode
pub fn disable_output() {
    unsafe {
        OUTPUT = false;
    }
}

/// Prints a message to stdout if output is enabled
pub fn print(msg: String) {
    unsafe {
        if OUTPUT {
            println!("{}", msg);
        }
    }
}

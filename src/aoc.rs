pub mod day;
pub mod input;
pub mod output;
pub mod part;

#[cfg(feature = "online")]
mod online;

pub mod cli;

#[cfg(feature = "wasm")]
pub mod wasm;

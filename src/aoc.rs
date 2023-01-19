pub mod day;
pub mod input;
pub mod output;
pub mod part;
pub mod results;
pub mod run;

#[cfg(feature = "online")]
mod online;

pub mod cli;

#[cfg(feature = "wasm")]
pub mod wasm;

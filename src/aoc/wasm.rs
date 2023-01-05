use wasm_bindgen::prelude::wasm_bindgen;

use crate::Solutions;

use super::results::Results;

#[cfg(all(feature = "online", target_arch = "wasm32"))]
compile_error!("feature \"online\" cannot be enabled with \"wasm\" at the same time. Use \"...  --no-default-features --features wasm\" instead.");
#[cfg(all(feature = "parallel", target_arch = "wasm32"))]
compile_error!("feature \"parallel\" cannot be enabled with \"wasm\" at the same time. Use \"...  --no-default-features --features wasm\" instead.");

#[wasm_bindgen]
#[allow(dead_code)]
pub fn years() -> Vec<u16> {
    Solutions::years()
}

#[wasm_bindgen]
#[allow(dead_code)]
pub fn days_for_year(year: u16) -> Vec<u8> {
    Solutions::days_for_year(year)
}

#[wasm_bindgen]
#[allow(dead_code)]
pub fn run(year: u16, day: u8, part: &str, input: &str) -> WasmResults {
    Solutions::run(year, day, &part.parse().unwrap(), &mut input.as_bytes()).into()
}

#[wasm_bindgen(getter_with_clone)]
pub struct WasmResults {
    pub part1: Option<String>,
    pub part2: Option<String>,
}

impl From<Results> for WasmResults {
    fn from(results: Results) -> Self {
        Self {
            part1: results.part1,
            part2: results.part2,
        }
    }
}

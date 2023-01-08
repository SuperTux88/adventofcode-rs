use adventofcode::aoc::{output, wasm::app::App};

#[cfg(all(feature = "online", target_arch = "wasm32"))]
compile_error!("feature \"online\" cannot be enabled with \"wasm\" at the same time. Use \"...  --no-default-features --features wasm\" instead.");
#[cfg(all(feature = "parallel", target_arch = "wasm32"))]
compile_error!("feature \"parallel\" cannot be enabled with \"wasm\" at the same time. Use \"...  --no-default-features --features wasm\" instead.");

pub fn main() {
    yew::Renderer::<App>::new().render();
    output::println("🎄 Advent of Code - WASM".to_string());
}

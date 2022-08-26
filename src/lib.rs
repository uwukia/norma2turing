mod utils;

mod logic;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen(getter_with_clone)]
pub struct CompilationResult {
    pub successful: bool,
    pub info: String,
    pub filedata: String,
    pub filename: String,
}

/// Returns: (compiled_successfully, compilation_info, compiled_file)
#[wasm_bindgen]
pub fn compile_code(input: &str) -> CompilationResult {
    logic::compile(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

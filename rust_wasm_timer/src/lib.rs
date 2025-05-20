use wasm_bindgen::prelude::*;

// JSのperformance.now()を呼ぶためのimport
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = performance)]
    fn now() -> f64;
}

// Rustから呼べる関数をエクスポート
#[wasm_bindgen]
pub fn measure_now_overhead(iterations: u32) -> f64 {
    let start = now();
    let mut end = start;

    for _ in 0..iterations {
        end = now();
    }

    end - start

}
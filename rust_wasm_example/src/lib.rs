use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32{
    a+b
}

#[wasm_bindgen]
pub fn run_loop(n: u32){
    for _ in 0..n{
        //何もしないループ
    }
}
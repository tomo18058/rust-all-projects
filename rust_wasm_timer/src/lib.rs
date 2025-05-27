use wasm_bindgen::prelude::*;

// JSのperformance.now()を呼ぶためのimport

// RustからJavaScriptに公開する関数を定義
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = performance)]// JSの `performance.now()` をRustで `now()` という関数として使えるようにする
    fn now() -> f64;
}

//JavaScriptから中断指示を出せるようにする
#[wasm_bindgen]
pub fn stop_measurement(){
    SHOULD_STOP.store(true, Ordering::Relaxed);
}

// Rustから呼べる関数をエクスポート
#[wasm_bindgen]
pub fn measure_now_overhead(iterations: u64) -> f64 {
    SHOULD_STOP.store(false, Ordering::Relaxed);  // 開始前にリセット

    let start = now();// 最初の時刻を記録
    let mut end = start;

    // `performance.now()` を `iterations` 回だけ呼び出す
    for _ in 0..iterations { 
        f SHOULD_STOP.load(Ordering::Relaxed) {
            break;  // 中断フラグが立っていたらループ終了
        }
        end = now();// 毎回上書きされるが、処理時間の合計を計る意図がある
    }

    // 最初と最後の時刻の差を返す
    // これが、`performance.now()` を `iterations` 回呼び出したことによる時間（オーバーヘッド）になる
    end - start

}
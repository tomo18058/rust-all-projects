// wasm_bindgen: RustコードをJavaScript(WASM)とつなぐための属性などを提供
use wasm_bindgen::prelude::*;

// js_sys::Date: JavaScriptのDate.now()相当。現在時刻(ms)を取得するのに使う
use js_sys::Date;

// rand: 乱数生成に使う（WebAssemblyでは cargo.toml に features=["std", "std_rng", "getrandom"] を指定しておくこと）
use rand::Rng;

#[wasm_bindgen] // JavaScriptから呼び出せるように公開する関数
pub fn run_sort_once(len: usize) -> Vec<f64> {
    // 0〜1の乱数値をlen個生成してベンチマーク用データとする
    let data = generate_data(len);

    // -------------------------
    // バブルソート計測
    // -------------------------
    let bubble_start = now();
    let mut copy = data.clone(); // オリジナルデータを複製して破壊的ソート可能に
    bubblesort(&mut copy);       // 実行
    let bubble_time = now() - bubble_start; // 所要時間(ms)

    // -------------------------
    // クイックソート計測
    // -------------------------
    let quick_start = now();
    let mut copy2 = data.clone();
    quicksort(&mut copy2);
    let quick_time = now() - quick_start;

    // JavaScriptにVec<f64>として返す（WASM側では自動的にJS配列になる）
    vec![bubble_time, quick_time]
}

// ランダムな0〜1の浮動小数点数をlen個生成する
fn generate_data(len: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    (0..len).map(|_| rng.r#gen::<f64>()).collect()
}

// バブルソート：後ろから前に比較しながらソート
fn bubblesort(arr: &mut [f64]) {
    for i in 0..arr.len() {
        for j in (i + 1..arr.len()).rev() {
            if arr[j] < arr[j - 1] {
                arr.swap(j, j - 1);
            }
        }
    }
}

// クイックソート（分割して再帰）
fn quicksort(arr: &mut [f64]) {
    if arr.len() <= 1 {
        return;
    }
    quicksort_rec(arr, 0, arr.len() - 1);
}

// クイックソートの再帰本体（左右からpivotを比較して中央に寄せる）
fn quicksort_rec(arr: &mut [f64], left: usize, right: usize) {
    if left >= right {
        return;
    }

    let pivot = arr[(left + right) / 2];
    let (mut i, mut j) = (left, right);

    while i <= j {
        while arr[i] < pivot {
            i += 1;
        }
        while arr[j] > pivot {
            if j == 0 { break; } // アンダーフロー防止
            j -= 1;
        }
        if i <= j {
            arr.swap(i, j);
            i += 1;
            if j == 0 { break; }
            j -= 1;
        }
    }

    if left < j {
        quicksort_rec(arr, left, j);
    }
    if i < right {
        quicksort_rec(arr, i, right);
    }
}

// JavaScriptのDate.now()相当：現在時刻をms単位のf64として取得
fn now() -> f64 {
    Date::now()
}

use std::collections::HashMap;

pub fn run() {
    let mut scores = HashMap::new();

    // 挿入
    scores.insert("Alice", 50);
    scores.insert("Bob", 80);
    scores.insert("Charlie", 30);

    println!("scores = {:?}", scores);

    // get(Option)
    if let Some(score) = scores.get("Bob") {
        println!("Bobのスコア = {}", score);
    }

    // 上書き
    scores.insert("Alice", 999);

    // for文で回す(順番確証なし)
    for (name,score) in &scores {
        println!("{} → {}", name, score);
    }
}
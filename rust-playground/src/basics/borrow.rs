pub fn run() {
    let s = String::from("Rust");

    // 参照 (借用)する
    print_length(&s);

    // 借用では所有権は移動しない
    println!("元のsもまだ使える:{}",s );
}

fn print_length(s: &String) {
    println!("{}の長さは{}", s, s.len());
}
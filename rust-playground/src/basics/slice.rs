pub fn run() {
    let s = String::from("Hello Rust!");

    //スライス
    let hello = &s[0..5];
    let rust = &s[6..10];

    println!("hello = {}", hello);
    println!("rust = {}", rust);

    //配列スライス
    let arr = [10, 20, 30, 40, 50];
    let slice = &arr[1..4];
    println!("slice = {:?}", slice);
}
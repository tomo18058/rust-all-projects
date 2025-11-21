pub fn run() {
    let s1 = String::from("Hello");
    let s2 = s1; //所有権がmoveする

    println!("s2 = {}", s2);

}
pub fn run() {
    let mut v = vec![10, 20, 30];

    // push
    v.push(40);

    println!("v = {:?}", v);

    //要素アクセス
    println!("1つ目 = {}", v[0]);

    // get()はOptionを返す安全なアクセス
    match v.get(10){
        Some(x) => println!("値 = {}", x),
        None => println!("10番目の要素は存在しません"),
    }

    // イテレーション
    for n in &v {
        println!("ループ = {}", n);
    }
}
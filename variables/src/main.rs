fn main() {
    // addition
    // 足し算
    let sum = 5 + 10;
    println!("{}",sum);

    // subtraction
    // 引き算
    let difference = 95.5 - 4.3;
    println!("{}",difference);

    // multiplication
    // 掛け算
    let product = 4 * 30;
    println!("{}",product);

    // division
    // 割り算
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
                              // 結果は0
    println!("{}",quotient);
    println!("{}",floored);

    // remainder
    // 余り
    let remainder = 43 % 5;
    println!("{}",remainder);

    
    // 文字型
    let a = 'z';
    let b = 'Z';
    let heart_eyed_cat = '😻';
    println!("{}",a);
    println!("{}",b);
    println!("{}",heart_eyed_cat);

    let tup = (500, 6.4, 1);
    let (c,d,e) = tup;

    println!("The value of y is: {}",c);
    println!("The value of y is: {}",d);
    println!("The value of y is: {}",e);    

    let tup:(i32,f64,u8) = (500,6.4,1);

    let f = tup.0;
    let g = tup.1;
    let h = tup.2;

    println!("x = {}", f);
    println!("g = {}", g);
    println!("h = {}", h);
}


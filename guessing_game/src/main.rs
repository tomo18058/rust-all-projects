use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is: {}",secret_number);

    loop {

        println!("Please input your guess.");

        let mut guess = String::new();// 入力値を保持する可変の文字列を作成


        io::stdin()
            .read_line(&mut guess) // 入力を受け取り guess に格納
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
            // .expect("Please type a number!");

        println!("You guessed: {}",guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),   //小さすぎ！
            Ordering::Greater => println!("Too big!"),  //大きすぎ！
            Ordering::Equal => {
                println!("You win!");  //勝ち！
                break;
            }
        }
    }
}

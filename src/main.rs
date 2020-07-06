extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is {}", secret_number);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    // コロン ":" でコンパイラへ型注釈を行う
    // guessに含まれるホワイトスペース(改行コード"\n"含む)を削除し別の型へ変換
    // 成功したらOk列挙子(自分自身=u32型のguess）を返す
    // 失敗したらErr列挙子(Panicと引数のメッセージ)を返す

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

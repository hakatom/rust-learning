use std::io;
use rand::Rng;
fn main() {
    println!("guess the number!");
    println!("please input your guess.");
    let mut guess = String::new();

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");
    io::stdin().read_line(&mut guess).expect("failed to read line");
    println!("you guessed: {guess}");
}

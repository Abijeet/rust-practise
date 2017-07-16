extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1, 101);
  let mut total_guesses = 0;
  loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
      .expect("Failed to read line.");

    println!("You guessed: {}", guess);

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    match guess.cmp(&secret_number) {
      Ordering::Less => {
        println!("Too small!");
        total_guesses += 1;
      }
      Ordering::Greater => {
        println!("Too big!");
        total_guesses += 1;
      }
      Ordering::Equal => {
        println!("You win!");
        total_guesses += 1;
        println!("You guessed a total of {} time(s).", total_guesses);
        break;
      }
    }
  }
}

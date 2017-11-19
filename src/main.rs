extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  let mut random_number_generator = rand::thread_rng();
  let secret_number = random_number_generator.gen_range(1, 101);

  println!("Guess the number!");
  println!("The secret number is {}", &secret_number);

  loop {
    println!("Please input your guess: ");  
    let mut guess = String::new();
    let stdin = io::stdin();
    let read_line_result = stdin.read_line(&mut guess);
    read_line_result.expect("Failed to read line.");

    println!("You guessed: {}", guess);

    let guess:u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };
      
      // .expect("Please type a number !");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => { 
            println!("You win!");
            break;
          }, 
    }
  }
}


extern crate rand; //declared rand in dependencies so we can use it here

use std::io;//import file io
use std::cmp::Ordering;//will use cmp method to compare secret num to user guess
use rand::Rng;//need Rng to be in scope when we use our rand crate

fn main() {
  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1, 101); //set secret_number to random var

  loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
      .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };//convert user guess into an int and ask again if input cannot be converted
        

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
          Ordering::Less    => println!("Too small!"),
          Ordering::Greater => println!("Too big!"),
          Ordering::Equal   => {
            println!("You win!"); 
            break; //end the game when user guesses correctly
      }      
    }
  }    
}

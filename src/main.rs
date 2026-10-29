use std::io;
use rand::prelude::*;
use std::cmp::Ordering;

fn main() {
    println!("welcome to my guessing game");
    
    loop {

        println!("please enter your guess");

        let secret_number = rand::rng().random_range(1..=100);
        let mut user_input = String::new();


        io::stdin()
            .read_line(&mut user_input)
            .expect("failed to take user input");

        //let user_input: u32 = user_input.trim().parse().expect("please enter a number");

        let user_input: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match user_input.cmp(&secret_number) {
            Ordering::Less => println!("this is less than the secret number"),
            Ordering::Greater => println!("this is greater"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }

}
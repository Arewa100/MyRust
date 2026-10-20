use std::io;

fn main() {
    println!("u welcome my guessing game");

    println!("please enter your guess");

    let mut user_input = String::new(); 

    io::stdin()
        .read_line(&mut user_input)
        .expect("failed to take user input");

    println!("you guessed {user_input}");

}
use std::io;
use std::cmp::Ordering;
use rand::Rng;

// function
fn main() {
    println!("THIS IS GUESSING GAME !");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret_number is : {secret_number}");
    
    println!("Hey, Enter your guessed number here --> ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too Small Guess !!!"),
        Ordering::Equal => println!("You Win !!!"),
        Ordering::Greater => println!("Too Big Guess"),
    }
}

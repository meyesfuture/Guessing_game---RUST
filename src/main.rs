use std::io;
use rand::prelude::*;

fn main() {
    println!("Guessing Game v1");
    println!("-----------------");

    let secret_number = rand::rng().random_range(1..=100);

    let mut guess = String::new();
    let mut chances = 5;
    
    

    while chances > 0{

        guess.clear();


        io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");


        let number: i32 = guess
        .trim()
        .parse()
        .expect("Please enter a valid integer");


        if number == secret_number {
            println!("congratulations! you guessed the correct number, which was {secret_number}");
            return;
        }
        else if number > secret_number {
            println!("try lower");
        }
        else{
            println!("try higher");
        }

        chances -= 1;
    }

    println!("you lost! the secret number was {secret_number}");

}

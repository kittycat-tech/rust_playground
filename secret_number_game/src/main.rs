use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Generating your random secret number!!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Your random secret number is: {secret_number}");

    println!(
        "Now it's your friend's turn to guess your secret number! Ask your friend to enter his/her guess. "
    );

    loop {
        println!("The secret number is a valid integer between 1 to 100");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number from 1 to 100!");
                continue;
            }
        };

        println!("Your friend's guess is: {guess}");

        println!("Now let's check if your friend's guess matches the secret number.");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("That is less than the secret number! Try again: "),
            Ordering::Greater => println!("That is more than the secret number! Try again: "),
            Ordering::Equal => {
                println!("Bingo! Your guess is correct! The secret number is :  {secret_number}");
                break;
            }
        }
    }
}

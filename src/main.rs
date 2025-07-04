use std::io;

fn main() {
    println!("This is a guessing game");
    println!("Guess the number");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("You failed to guess the number");

    println!("You guessed: {guess}");
}

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the guessing game!");
    let mut correct = false;
    let secret_number = rand::thread_rng().gen_range(1, 101);

    while !correct {
        play_round(&secret_number, &mut correct);
    }
}

fn play_round(secret_number: &u32, correct: &mut bool) {
    println!("Enter your guess: ");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    let guess: u32 = guess.trim().parse().expect("Could not parse input!");

    println!("You guessed: {}", &guess);
    compare_guess(&guess, &secret_number, correct);
}

fn compare_guess(guess: &u32, secret_number: &u32, correct: &mut bool) {
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too low!"),
        Ordering::Equal => {
            *correct = true;
            println!("Correct!")
        }
        Ordering::Greater => println!("Too high!"),
    }
}

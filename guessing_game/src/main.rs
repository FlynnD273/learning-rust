use std::io;
use std::cmp::Ordering;
use rand::Rng;

enum GameType {
   PlayerGuess,
   ComputerGuess,
}

fn main() {
    let mut game_response = String::new();

    println!("Should the player guess the number? (y/n)");

    io::stdin()
            .read_line(&mut game_response)
            .expect("Failed to read line");

    let is_player_guess = game_response.trim().to_lowercase() == "y";
    let game_type; 

    if is_player_guess {
        game_type = GameType::PlayerGuess;
    } 
    else {
        game_type = GameType::ComputerGuess;
    }

    let guess_count: u32;

    let min = 1;
    let max = 100;

    match game_type {
        GameType::ComputerGuess => guess_count = computer_guess(min, max),
        GameType::PlayerGuess => guess_count = player_guess(min, max),
    }

    println!("Took {guess_count} guesses!");
}

fn computer_guess(guess_min: i32, guess_max: i32) -> u32 {
    let mut guess_count: u32 = 0;
    let mut min = guess_min;
    let mut max = guess_max;

    println!("Pick a number from {min} to {max}");

    loop {
        let guess = (min + max) / 2;

        println!("Is your number {guess}? (h/l/c)");

        let mut response = String::new();

        io::stdin()
            .read_line(&mut response)
            .expect("Failed to read line");

        let response = response.trim().to_lowercase();

        guess_count += 1;

        match response.as_str() {
            "h" => min = guess + 1,
            "l" => max = guess - 1,
            "c" => break,
            _ => {
                println!("Invalid response!");
                guess_count -= 1;
                continue;
            }
        }

        if min > max {
            println!("You're cheating!");
            min = guess_min;
            max = guess_max;
            guess_count = 0;
        }
    }

    return guess_count;
}

fn player_guess(min: i32, max: i32) -> u32 {
    let mut guess_count: u32 = 0;
    let secret_number = rand::thread_rng().gen_range(min..=max);

    println!("Guess the number!");

    loop {
        println!("Enter your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number!");
                continue;
            }
        };

        println!("Your guess was {guess}");

        guess_count += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Higher!"),
            Ordering::Greater => println!("Lower!"),
            Ordering::Equal => {
                    println!("Correct!");
                    break;
                },
        }
    }

    return guess_count;
}

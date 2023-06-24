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

    let guess_count;

    match game_type {
        GameType::ComputerGuess => guess_count = computer_guess(),
        GameType::PlayerGuess => guess_count = player_guess(),
    }

    println!("Took {guess_count} guesses!");
}

fn computer_guess() -> u32 {
    // Implement computer guessing
    println!("The computer's guessing!");
    return 0;
}

fn player_guess() -> u32 {
    let mut guess_count = 0;
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number!");
                continue;
            }
        };

        println!("Your guess was {guess}");

        guess_count += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                    println!("Correct!");
                    break;
                },
        }
    }

    return guess_count;
}

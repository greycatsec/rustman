use std::io;
use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use rand::seq::SliceRandom;

fn main() {
    let mut won_game = false;
    let mut quit_game = false;
    println!("Welcome to Rustman, a Rust Hangman game.");
    let secret_word = select_word();
    let mut guessed_characters: Vec<char> = Vec::new();
    let mut masked_word = mask_word(&secret_word, &guessed_characters);
    let mut allotted_misses = 10;

    loop {
        println!("Progress: {}", masked_word);
        let stringuesses: String = guessed_characters.iter().collect();
        println!("You have {} guesses remaining and you've guessed [{}] so far.", allotted_misses, stringuesses);
        println!("Enter a character to guess (or '!' to quit): ");
        let mut guess = String::new();
        match io::stdin().read_line(&mut guess) {
            Ok(_) => {
            // received input, continue with game
                let guessed_chara = guess.trim().chars().next(); // get the first character of their input

                match guessed_chara {
                    Some('!') => {
                        quit_game = true;
                        break // quit game when ! is entered
                    }
                    Some(c) => {
                        guessed_characters.push(c);
                        if !(secret_word.contains(c)) {
                            allotted_misses -= 1;
                            if allotted_misses == 0 {
                                break;
                            }
                        }
                        masked_word = mask_word(&secret_word, &guessed_characters);
                    }
                    _ => continue, // ignore invalid input
                }
                if win_condition(&masked_word) {
                    won_game = true;
                    break;
                }
            }
            Err(err) => {
            println!("Received error: {}", err);
            }
        }
    }
    if won_game {
        println!("Congrats! You won the game! The word was '{}'", secret_word)
    }
    else if quit_game {
        println!("You quit the game.")
    } else {
        println!("You lost the game! The word was '{}'", secret_word)
    }

}

fn mask_word(secret_word:&str, guessed_charas:&Vec<char>) -> String {
    let mut masked = String::new();
    for chara in secret_word.chars() {
        if guessed_charas.contains(&chara) {
            masked.push(chara);
        } else {
            masked.push('_');
        }
    }
    masked
}

fn win_condition(masked_word: &str) -> bool {
    if !(masked_word.contains('_')) {
        return true;
    }
    return false;
}


fn select_word() -> String {
    let file_path = Path::new("wordlist.txt");
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let words: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let mut rng = rand::thread_rng();
    let mut random_word = words.choose(&mut rng);

    while random_word.map(|word| word.len()).unwrap_or(0) < 4 {
        random_word = words.choose(&mut rng);
    }

    random_word.unwrap().to_string()

}
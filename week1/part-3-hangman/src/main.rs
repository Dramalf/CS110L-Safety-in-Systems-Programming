// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;
const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}
fn get_guess() -> char {
    loop {
        print!("Please enter your guess: ");
        io::stdout().flush().unwrap();
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess = guess.trim().parse::<char>();
        match guess {
            Ok(ch) => return ch,
            Err(_) => println!("Invalid input. Please enter a single character."),
        }
    }
}

fn update_display_word(word: &str, display_word: &mut Vec<char>, guess: char) {
    for (i, ch) in word.chars().enumerate() {
        if ch == guess {
            display_word[i] = ch;
        }
    }
}

fn print_display_word(display_word: &[char]) {
    for ch in display_word {
        print!("{} ", ch);
    }
    println!();
}

fn main() {
    let secret_word = pick_a_random_word();
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    let mut display_word: Vec<char> = vec!['_'; secret_word.len()];
    let mut incorrect_guesses = 0;


    while incorrect_guesses < NUM_INCORRECT_GUESSES {
        print_display_word(&display_word);
        let guess = get_guess();
        if secret_word_chars.contains(&guess) {
            update_display_word(&secret_word, &mut display_word, guess);
            if secret_word== display_word.iter().collect::<String>() {
            
                break;
            }
        } else {
            incorrect_guesses += 1;
            println!("Incorrect guess! You have {} incorrect guesses remaining.", NUM_INCORRECT_GUESSES - incorrect_guesses);
        }
    }

    if incorrect_guesses == NUM_INCORRECT_GUESSES {
        println!("You lost! The secret word was: {}", secret_word);
    } else {
        println!("Congratulations! You won!");
    }
}

mod constants;
use crate::constants::*;
use rand::Rng;
use std::io;

#[derive(Debug)]
struct Game {
    secret: String,
    guessed: [bool; 26],
    wrong: u32,
}

enum GuessResult {
    AlreadyGuessed,
    Hit,
    Miss,
}

impl Game {
    fn new(words: &[&str]) -> Self {
        Self {
            secret: words[rand::thread_rng().gen_range(0..words.len())].to_string(),
            guessed: [false; 26],
            wrong: 0,
        }
    }

    fn guess(&mut self, letter: char) -> GuessResult {
        let letter_index = letter.to_ascii_lowercase() as usize - 'a' as usize;
        if self.guessed[letter_index] {
            return GuessResult::AlreadyGuessed;
        }
        for character in self.secret.chars() {
            self.guessed[letter_index] = true;
            if character.eq_ignore_ascii_case(&letter) {
                return GuessResult::Hit;
            }
        }
        GuessResult::Miss
    }

    fn is_won(&self) -> bool {
        self.secret.chars().all(|letter| {
            let index = (letter.to_ascii_lowercase() as usize) - ('a' as usize);
            self.guessed[index]
        })
    }
    fn is_lost(&self) -> bool {
        self.wrong >= 6
    }
    fn render(&self) -> String {
        let mut result = String::new();
        for letter in self.secret.chars() {
            let i = (letter.to_ascii_lowercase() as u8 - b'a') as usize;
            if self.guessed[i] {
                result.push(letter);
            } else {
                result.push('_');
            }
            result.push(' ');
        }
        result
    }
    fn gallows(&self) -> &'static str {
        match self.wrong {
            0 => STAGE_0,
            1 => STAGE_1,
            2 => STAGE_2,
            3 => STAGE_3,
            4 => STAGE_4,
            5 => STAGE_5,
            6 => STAGE_6,
            _ => "nothing",
        }
    }
}

fn main() {
    let mut game = Game::new(&WORDS);

    loop {
        println!("{}", game.gallows());
        println!("Word :    {}\n", game.render());
        println!("Wrong:    {}/6", game.wrong);

        if game.is_lost() {
            println!(
                "You ran out of wrong guesses! You lost! The word was {0}",
                game.secret
            );
            break;
        } else if game.is_won() {
            println!("You guessed the word! You won!");
            break;
        }

        let mut guessed_letter = String::new();
        let valid_letter: char = loop {
            guessed_letter.clear();
            println!("Enter a letter:    ");
            io::stdin()
                .read_line(&mut guessed_letter)
                .expect("Failure to read the guessed letter");

            let trimmed_letter = guessed_letter.trim();

            if trimmed_letter.chars().count() == 1 {
                let letter = trimmed_letter.chars().next().unwrap();

                if letter.is_ascii_alphabetic() {
                    break letter;
                }
            }

            println!("This is not a valid letter. Try Again!");
        };


        match game.guess(valid_letter) {
            GuessResult::AlreadyGuessed => {
                println!("This letter is guessed already, Try another one!");
                continue;
            }
            GuessResult::Hit => {
                println!("Good Guess!")
            }
            GuessResult::Miss => {
                println!("Wrong Guess!");
                game.wrong += 1;
                continue;
            }
        }
    }
}

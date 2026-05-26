mod constants;
use crate::constants::*;
use rand::Rng;
use std::fmt::Write;
use std::io;

// data required for game : word bank, guesses(lives), letters guessed

#[derive(Debug)]
struct Game {
    secret: String,
    guessed: [bool; 26],
    wrong: u32,
}

impl Game {
    fn new(words: &[&str]) -> Self {
        Self {
            secret: words[rand::thread_rng().gen_range(0..=words.len())].to_string(),
            guessed: [false; 26],
            wrong: 0,
        }
    }
    // fn was_guessed(&self, letter: char) -> bool {
    //     let letter_letter.to_ascii_lowercase() as usize - 'a' as usize;
    //     for character in self.guessed {
    //     }
    //     true
    // }
    // fn guess(&mut self, letter: char) -> bool {}
    // fn is_won(&self) -> bool {}
    // fn is_lost(&self) -> bool {}
    fn render(&self) -> String {
        let mut result = String::new();
        for letter in self.secret.chars() {
            let i = (letter as u8 - b'a') as usize;
            if self.guessed[i] {
                write!(result, "{} ", letter);
            } else {
                write!(result, "{}", "_ ");
            }
        }

        result
    }
    fn gallows(&self) -> &'static str {
        match self.wrong {
            0 => &STAGE_0,
            1 => &STAGE_1,
            2 => &STAGE_2,
            3 => &STAGE_3,
            4 => &STAGE_4,
            5 => &STAGE_5,
            6 => &STAGE_6,
            _ => "nothing",
        }
    }
}

fn main() {
    let game = Game::new(&WORDS);
    dbg!(&game);

    loop {
        println!("{}", game.gallows());
        println!("Word :    {}\n", game.render());
        println!("Wrong:    {}/6", game.wrong);

        let mut guessed_letter = String::new();
        let valid_letter: char = loop {
            guessed_letter.clear();
            print!("Enter a letter:    ");
            io::stdin()
                .read_line(&mut guessed_letter)
                .expect("Failure to read the guessed letter");

            let trimmed_letter = guessed_letter.trim();

            if trimmed_letter.chars().count() == 1 {
                let letter = trimmed_letter.chars().next().unwrap();

                if letter.is_alphabetic() {
                    break letter;
                }
            }

            println!("This is not a valid letter. Try Again!");
        };

        println!("{}", &guessed_letter);
        println!("{}", game.was_guessed('a'));

        break;
    }
}

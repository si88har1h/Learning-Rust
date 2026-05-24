use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Shhh... Generating your secret number");
    let secret: i32 = rand::thread_rng().gen_range(1..=100);
    let mut tries: i32 = 0;

    loop {
        let mut guess = String::new();
        println!("<<<<<<Enter a number>>>>>");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read your number");

        tries += 1;

        let guess: i32 = match guess.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Error parsing the number, enter a valid number!");
                continue;
            }
        };

        match guess.cmp(&secret) {
            Ordering::Greater => {
                if guess - secret > 20 {
                    println!("Way too Big!");
                } else if guess - secret > 10 {
                    println!("Little Less Big!");
                } else {
                    println!("wow getting there!");
                }
            }
            Ordering::Less => {
                if secret - guess > 20 {
                    println!("Way too Small!");
                } else if secret - guess > 10 {
                    println!("Smaller but not that much!");
                } else {
                    println!("wow getting there but a bit higher!");
                }
            }
            Ordering::Equal => {
                println!(
                    "Voila! You guessed it right in {tries} {} ! Its {secret}",
                    if tries > 1 { "turns" } else { "turn" }
                );
                break;
            }
        }
    }
}

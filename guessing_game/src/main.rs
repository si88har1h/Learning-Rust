use rand::Rng;
use std::io;

fn main() {
    println!("Shhh... Generating your secret number");
    let secret: i32 = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        println!("<<<<<<Enter a number>>>>>");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read your number");

        let parsed_guess: i32 = match guess.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Error parsing the number, defaulting to 0");
                0
            }
        };
        let direction = if parsed_guess > secret {
            "Too High"
        } else {
            "Too Low"
        };
        if parsed_guess == secret {
            println!("Great Success! Your secret was {secret}");
            break;
        } else if parsed_guess.abs_diff(secret) < 2 {
            println!("Almost missed it!!! {direction}");
        } else if parsed_guess.abs_diff(secret) < 5 {
            println!("Soo close yet so far! {direction}");
        } else if parsed_guess.abs_diff(secret) < 10 {
            println!("Closing in! {direction}");
        } else if parsed_guess - secret > 20 {
            println!(" Wayyy {direction}");
        } else if parsed_guess - secret > 10 {
            println!("Aim a little low!");
        } else if secret - parsed_guess > 20 {
            println!(" wayyy {direction}!");
        } else if secret - parsed_guess > 10 {
            println!("Aim a little high!");
        } else {
            println!("Oops wrong guess! Try Again!")
        }
    }
}

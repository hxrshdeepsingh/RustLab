use std::io;
use rand::RngExt;

fn main() {
    println!("gues the game");
    println!("choose number between 1 to 100");

    let num = rand::rng().random_range(1..=100);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess);
        let user_guess = guess.trim().parse::<u32>().unwrap();

        if user_guess > num {
            println!("high");
            continue;
        } else if user_guess < num {
            println!("low");
            continue;
        } else {
            println!("you mew~~~");
            break;
        }
    }
}
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!(" Secret number is {secret_number}");

    loop {
        println!("Provide your guessing number.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read no");

        println!("your guess is {}", guess);

        // let guess: u32 = guess.trim().parse().expect("Please type number");

        let guess: u32 = match guess.trim().parse() {
          Ok(num) => num,
          Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
        }
    }
}

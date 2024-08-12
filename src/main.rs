use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess our number or we'll delete your system");

    let secret = rand::thread_rng().gen_range(1..=100);

    println!("Psst! The number is: {}", secret);

    
    // let mut guess = String::new();
    // If you don't create a new string each time, read_line will concatenate all of em
    // not the desired result

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => { //Catchall phrase
                println!("Not a number dumbass! {}", guess);
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

    }
}

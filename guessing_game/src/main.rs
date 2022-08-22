extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        eprint!("Please input your guess. \n$ ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        
        if let Ok(guess) = guess.trim().parse::<u32>() {
            match guess.cmp(&secret_number) {
                Ordering::Less      => println!("Too small!"),
                Ordering::Greater   => println!("Too big!"),
                Ordering::Equal     => {
                    println!("You win!");
                    break;
                }
            }
        } else {
            println!("Please type a number!");
            continue;
        }

        // let guess: u32 = match guess.trim().parse() {
        //     Ok(num)     => {
        //         println!("You guessed {:?}.",num);
        //         num
        //     },
        //     Err(e)      => {
        //         println!("Error! Please type a number! {e:?}");
        //         continue;
        //     },
        // };

    }
}

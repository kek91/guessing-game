#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate rand;

use std::io;
use std::cmp::Ordering;
use std::time::Duration;
use std::thread;
use rand::Rng;

fn main() {
    while run() {
        println!("\n\nWelcome to the Guessing Game\n\n");
        let random_number = rand::thread_rng().gen_range(1,101);
        // println!("{}", random_number);
        let mut i : u16 = 0;
        loop {
            if i == 0 {
                println!("\n\nEnter your guess:");
            }
            else {
                println!("\n\nGuess again!");
            }
            let mut guess = String::new();
            io::stdin().read_line(&mut guess).expect("Failed to read line");
            // let guess : u32 = guess.trim().parse().expect("Error: Enter a number.");
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Numbers only...");
                    continue;
                }
            };
            // println!("You guessed {}", guess);
            match guess.cmp(&random_number) {
                Ordering::Less    => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal   => {
                    println!("You win!");
                    println!("Restarting game... please wait.");
                    thread::sleep(Duration::from_millis(2000));
                    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
                    break;
                }
            }

            /*
            if guess == random_number {
                println!("\n\nCongrats, you win! Press ENTER to quit or type R to replay.");
                let mut confirmExit = String::new();
                io::stdin().read_line(&mut confirmExit).expect("Failed to read line");
                if confirmExit != "" && confirmExit == "r" {
                    println!("Restarting...");
                }
                else if confirmExit != "" && confirmExit != "r" && confirmExit != "R" {
                    println!("Bye...");
                    std::process::exit(0);
                }
            }
            */
            i+=1;
        }
    }
}

fn run() -> bool {
    return true;
}

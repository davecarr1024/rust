use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let num = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("guess?");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("invalid input: {err}");
                continue;
            }
        };
        println!("guess: {guess}");
        match guess.cmp(&num) {
            Ordering::Less => println!("too low"),
            Ordering::Greater => println!("too high"),
            Ordering::Equal => {
                println!("got it");
                break;
            }
        }
    }
}

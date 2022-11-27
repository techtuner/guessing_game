use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut trials = 0;
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to load the content");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        print!("Enter the number: ");
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                trials = trials + 1;
                println!("Guess is lower");
            }
            Ordering::Equal => {
                println!("You win!!!");
                println!("You took {trials} to guess the number!!!!");
                break;
            }
            Ordering::Greater => {
                trials = trials + 1;
                println!("Guess is Higher");
            }
        }
    }
}

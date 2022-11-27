use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!(
        r#"                              _   _                                  _               "#
    );
    println!(
        r#"                             | | | |                                | |              "#
    );
    println!(
        r#"   __ _ _   _  ___  ___ ___  | |_| |__   ___   _ __  _   _ _ __ ___ | |__   ___ _ __ "#
    );
    println!(
        r#"  / _` | | | |/ _ \/ __/ __| | __| '_ \ / _ \ | '_ \| | | | '_ ` _ \| '_ \ / _ \ '__|"#
    );
    println!(
        r#" | (_| | |_| |  __/\__ \__ \ | |_| | | |  __/ | | | | |_| | | | | | | |_) |  __/ |   "#
    );
    println!(
        r#"  \__, |\__,_|\___||___/___/  \__|_| |_|\___| |_| |_|\__,_|_| |_| |_|_.__/ \___|_|   "#
    );
    println!(
        r#"   __/ |                                                                             "#
    );
    println!(
        r#"  |___/                                                                              "#
    );

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut trials = 1;
    loop {
        let mut guess = String::new();
        println!("Enter a number to guess: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to load the content");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                trials = trials + 1;
                println!("Guess is lower");
            }
            Ordering::Equal => {
                println!("You win!!!");
                println!("You took {trials} turns to guess the number!!!!");
                break;
            }
            Ordering::Greater => {
                trials = trials + 1;
                println!("Guess is Higher");
            }
        }
    }
}

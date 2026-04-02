use rand::Rng;
use std::io;


fn main() {
    println!("Welcome to the Memory Game!");
    println!("Try to remember the sequence of numbers.");
    let mut sequence: Vec<u32> = Vec::new();
    let mut round = 1;
    loop{
        println!("Round {}", round);

        let new_number = rand::thread_rng().gen_range(1..=9);
        sequence.push(new_number);

        println!("Memorize:");
        for num in &sequence{
            print!("{} ", num);
        }
        println!();

        thread::sleep(time::Duration::from_secs(2));

    }
}

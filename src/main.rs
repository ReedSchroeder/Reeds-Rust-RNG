use std::io;
use rand::Rng;

fn main() {
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed");

    let num_int: u32 = num.trim().parse().expect("Please type a number!");

    let roll = rand::thread_rng().gen_range(1..=num_int);

    println!("You rolled a {roll}");
}

use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Let's Play the Fucking Guessing Game Shall we?");

    println!("Stop with that BS and now input a random Number");

    let mut guess_num: String = String::new();
    let num: u32 = rand::thread_rng().gen_range(1..10);


    io::stdin()
        .read_line(&mut guess_num)
        .expect("You are so stupid you couldn't input a number");

    let guess_num: u32 = guess_num.trim().parse().expect("cum");

    match guess_num.cmp(&num) {
        Ordering::Less=>{println!("It's less")},
        Ordering::Greater=>{println!("It's Greater")},
        Ordering::Equal=>{println!("It's Equal")},
    }
}

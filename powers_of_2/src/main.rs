use std::io;
use std::cmp::Ordering;

fn main() {
    println!("What is the next power of 2?");

    let mut power : u32 = 1;
    let mut answer = u64::pow(2, power);

    loop {
        println!("Please type 2 ^ {}:", power);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&answer) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You got it!");
                power = power + 1;
                answer = u64::pow(2, power)
            }
        };
    }
}

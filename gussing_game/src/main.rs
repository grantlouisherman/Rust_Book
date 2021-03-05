use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("SECRET {}", secret_number);
loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to Read Line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering:: Less => println!("too small!"),
            Ordering:: Greater => println!("too big!"),
            Ordering:: Equal => {
                println!("YOU WIN MF!");
                break;
            }
        }
    };
}

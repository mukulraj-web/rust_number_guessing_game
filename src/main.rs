use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("enter the number you guess!");
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read the number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("the guessed number is: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("guessed low"),
            Ordering::Greater => println!("guessed higher"),
            Ordering::Equal => {
                println!("you are correct");
                break;
            }
        }
    }
}

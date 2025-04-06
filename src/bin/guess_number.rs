use rand::random_range;
use std::{cmp::Ordering, io};
fn main() {

    let secret_number=random_range(1..101);
    // println!("secret num is {}",secret_number);

    println!("Guess your number!");
    
    loop {
    println!("Please input your guess:");
    let mut guess: String=String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    
    // let guess:u32=guess.trim().parse().expect("pls type a number");
    let guess:u32=match guess.trim().parse() {
        Ok(num)=>num,
        Err(_)=>continue,
    };
    println!("You guessed:{}",guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Equal=>
            {
                println!("You win!");
                break;
            },
            Ordering::Greater=>println!("Too big"),
            Ordering::Less=>println!("Too small"),
        }
    
    }
}
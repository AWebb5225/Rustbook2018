use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess a number!"); //macro to print to stdout
    loop {
    println!("Enter your guess: ");
    let mut guess = String::new(); //let creates new variable guess "mut" makes it mutable
    io::stdin().read_line(&mut guess)
                 .expect("Failed to read line"); //read into the address of guess
    let guess: u32 = guess.trim().parse()
                 .expect("Please type a number!");
    println!("You guess: {}", guess);
    let rand_num = rand::thread_rng().gen_range(1,100);
    println!("The random number is: {}", rand_num);
    
    match guess.cmp(&rand_num) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("Just Right!"),
    }
}
}
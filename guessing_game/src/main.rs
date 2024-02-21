use core::num;
use std::{io, u32};
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the Number!");

    let y = rand::thread_rng().gen_range(1..101);
    println!("The real number is: {}", y);

    loop {
        println!("Please input number: ");

        let mut x = String::new();
    
        io::stdin()
        .read_line( &mut x)
        .expect("Failed to read line");
    
        let x: u32 = match x.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };
    
        println!("Your guess: {}", x);
    
        match x.cmp(&y){
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Greater => println!("{}","Too big!".red()),
            Ordering::Equal =>{ 
                println!("{}","You win!".green());
                break;
            }
        };  
    }

    
}

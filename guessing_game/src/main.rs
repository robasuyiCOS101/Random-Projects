use std::io;


fn main() {
    println!("Welcome to Russell's Guessing game");
    println!("You have three attempts to guess the number between 1 and 10,after which correct you go with a cash prize of $100,000 ");
    println!("\nEnter number");

    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Not a valid String");
    let num:i32 = num.trim().parse().expect("Not a valid number");
    
    



    if num > 7 {
        println!("Wrong!!The number is lower ");
    }else if num < 7 {
        println!("Wrong!!THe number is higher");
    } else if num == 7 {
        println!("You are wonderful
                  you are now the winner of the game and have been rewarded with $100000");
    }



          



}
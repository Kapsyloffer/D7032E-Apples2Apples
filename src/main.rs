use apples::game::*;
use std::io;
fn main() 
{
    println!("Hello world!");
    //TODO: Client and Server
    //Starts the game
    loop {
        println!("Menu:");
        println!("1. Option 1");
        println!("2. Option 2");
        println!("3. Option 3");
        println!("4. Exit");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim() {
            "1" => println!("Option 1 selected"),
            "2" => println!("Option 2 selected"),
            "3" => println!("Option 3 selected"),
            "4" => {
                println!("Exiting program");
                break;
            }
            _ => println!("Invalid option selected"),
        }
    }
    init_game();
}
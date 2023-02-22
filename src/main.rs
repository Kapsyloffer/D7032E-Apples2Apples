#![allow(dead_code)]
use apples::game::*;
use std::*;
use std::net::*;
use apples::networking::*;
fn main() 
{
    //menu_main();
    //TODO: Client and Server
    //Starts the game
    init_game();
}

fn menu_main()
{
    loop 
    {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("Apples To Apples:");
        println!("1. Join Lobby");
        println!("2. Create Lobby");
        println!("0. Exit");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim() 
        {
            "1" => loop 
            {
                let _ = join_lobby();
                break;
            },
            "2" => loop
            {
                let _ = host_lobby();
                break;
            },
            "0" => 
            {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                println!("Exiting program");
                break;
            }
            _ => println!("Invalid option selected"),
        }
    }
}

fn join_lobby()-> std::io::Result<()>
{
    loop 
    {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("0. <-- Go back");
        println!("---Join lobby---");
        print!("Please enter an IP:  ");
        match "127.0.0.1".parse::<Ipv4Addr>()
        {
            Ok(ip4) =>
            {
                let _c =  client_factory(ip4, 42069);
            },
            Err(e) =>
            {
                println!("Error message: {}", e.to_string());
            }
        }
    }
}

fn host_lobby()
{
    println!("Lobby hosted at: [PORT]");
    //Choose number of bots
    //MODE: Judge or vote?
    //Maybe ability to change points?
}
use std::*;
use std::net::*;
use crate::networking::*;
use crate::game::*;
use crate::settings::{default_settings, custom_settings};

extern crate colorize;
use colorize::*;


//A menu you can use before the game starts.
pub fn menu_main()
{
    loop 
    {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("Apples To Apples:");
        println!("1. Join Lobby (WIP)");
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
    //settings
    let mut j = true;
    let mut d = false;
    let mut w = 0;
    let mut b = 3;
    let mut modified = false;
    //println!("Lobby hosted at: [PORT]");
    loop 
    {
        //Clears the sceen
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        //Printstuff (I know it is ugly but it looks kinda good to the user okay?)
        println!("{}", "=== GAME SETUP ===".green());
        if !modified {
            println!("Press 0 to play ({})", if !modified {"default".to_string().green()} else {"modified".to_string().yellow()});
        }
        else{
            println!("Press 0 to play (modified)");
        }
        println!("Press 1 to toggle judge ({})", if j {j.to_string().green()} else {j.to_string().red()});
        println!("Press 2 to toggle discard phase ({})", if d {d.to_string().green()} else {d.to_string().red()});
        println!("Press 3 to set # of wild apples ({})", if w > 0 {w.to_string().green()} else {w.to_string().red()});
        println!("Press 4 to set # of bots ({}) (min 3)", if b == 3 {b.to_string().yellow()} else {b.to_string().green()});
        println!("\n=== press {} to return ===", "q".to_string().yellow());

        //Input handling
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim() 
        {
            "1" => 
            {
                j = !j;
            },
            "2" => 
            {
                d = !d;
            },
            "3" =>
            {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                //Input handling for "3"
                println!("How many wild apples do you want? Currently: {}", w.to_string());
                let mut input_3 = String::new();
                io::stdin().read_line(&mut input_3).expect("Failed to read line");
                match input_3.trim().parse::<i32>()
                {
                    Ok(num) =>
                    {
                        if num >= 0
                        {
                            w = num
                        }
                        else
                        {
                            w = 0;
                        }

                    },
                    Err(_) => println!("Not a number.")
                }
            }
            "4" =>
            {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                //Input handling for "3"
                println!("How many wild apples do you want? Currently: {}", b.to_string());
                let mut input_4 = String::new();
                io::stdin().read_line(&mut input_4).expect("Failed to read line");
                match input_4.trim().parse::<i32>()
                {
                    Ok(num) => 
                    {
                        if num >= 3
                        {
                            b = num
                        }
                        else
                        {
                            b = 3;
                        }
                    },
                    Err(_) => println!("Not a number.")
                }
            }
            "0" => 
            {
                if !modified
                {
                    init_game(default_settings());
                }
                else 
                {
                    init_game(custom_settings(j, d, w));
                }
                break;
            }
            "q" =>
            {
                break;
            }
            _ => println!("Invalid option selected"),
        }
        modified = true;

    }
    //Choose number of bots
    //MODE: Judge or vote?
    //Maybe ability to change points?
}
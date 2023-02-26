#![allow(unused_imports)]
#![allow(dead_code)]
use crate::card::*;
use std::net::TcpStream;
use std::io::*;
use rand::Rng;
use std::io;

extern crate colorize;
use colorize::*;

#[derive(Clone)]
pub struct Player //I guess all networking handlas av Client
{
    player_id : i32,
    is_bot : bool,
    online : bool,
    pub hand : Vec<RedCard>, //debugging, remove pub later
    green_apples : Vec<GreenCard>,
}

pub trait PlayerActions
{
    fn play_card (&mut self) -> RedCard;
    fn get_green_amount (&self) -> u8;
    fn add_to_hand(&mut self, rc : RedCard); 
    fn get_hand_size(&self) -> u8;
    fn get_id(&self) -> i32;
    fn give_green(&mut self, g : GreenCard);
    fn vote(&self, cards : &mut Vec<(i32, RedCard)>) -> i32; //returns the selected ID
}
pub trait Judge
{
   fn pick(&self, cards : &mut Vec<(i32, RedCard)>) -> i32; //returns the winner ID
}

impl Judge for Player
{
    fn pick(&self, cards : &mut Vec<(i32, RedCard)>) -> i32
    {
        let mut competitors : Vec<i32> = Vec::new();
        let c_size = &cards.len();
        
        for c in cards
        {
            println!(" {}: {}\n{}", competitors.len().to_string().yellow(), c.1.get_title().red().bold(), c.1.get_desc().red());
            competitors.push(c.0);
        }

        if self.is_bot
        {
            return competitors[rand::thread_rng().gen_range(0..competitors.len())];
        }
        else
        {
            loop 
            {
                println!("{}", "\n==YOU ARE THE JUDGE==\n".yellow().bold());
                println!("=== {} ===", "Pick the best card".cyan());
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");

                let _ = match input.trim().parse::<usize>() 
                {
                    Ok(num) if num < *c_size => 
                    {
                        return competitors[num];
                    },
                    _ => 
                    {
                        println!("Invalid input. Please try again.");
                        continue;
                    }
                };
            }
        }
    }
}


impl PlayerActions for Player
{
    //Used for drawing cards.
    fn add_to_hand(&mut self, rc : RedCard) 
    {
        self.hand.push(rc);
    }

    //If this player won the turn, give them the green.
    fn give_green(&mut self, g : GreenCard) 
    {
        self.green_apples.push(g);
    }

    fn get_green_amount (&self) -> u8 
    {
        let greens : u8 = self.green_apples.len() as u8;
        return greens;
    }

    fn get_hand_size(&self) -> u8 
    {
        let handsize : u8 = self.hand.len() as u8;
        return handsize;
    }

    fn get_id(&self) -> i32
    {
        return self.player_id;
    }

    fn play_card (&mut self) -> RedCard 
    {
        //if bot, do random, else let the player pick
        if self.is_bot
        {
            return self.hand.remove(rand::thread_rng().gen_range(0..self.hand.len()));
        }
        else
        {
            let mut i = 0;
            //Print out each card in hand
            for c in self.hand.iter_mut()
            {
                println!("{}: {}\n -{}", i.to_string().yellow(), c.get_title().red().bold(), c.get_desc().red());
                i+=1;
            }
            //Then let the player select em.
            loop 
            {
                println!("\n === {} ===", "Pick your card".cyan());
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");

                let _ = match input.trim().parse::<usize>() 
                {
                    Ok(num) if num < self.hand.len() => 
                    {
                        return self.hand.remove(num);
                    },
                    _ => 
                    {
                        println!("Invalid input. Please try again.");
                        continue;
                    }
                };
            }

        }
        //foreach card in hand, print: THESE ARE YOUR CARDS; 1. {title} {desc}, 2. ...
        //ask for input,
    }

    fn vote(&self, cards : &mut Vec<(i32, RedCard)>) -> i32 //TODO: TEST
    {
        let mut competitors : Vec<i32> = Vec::new();
        let c_size = &cards.len();
        
        //print out each card not played by the player (random ofc)
        for c in cards
        {
            if c.0 != self.get_id()
            {
                println!("{}:\n{}\n {}", competitors.len().to_string(), c.1.get_title(), c.1.get_desc());
                competitors.push(c.0); //push ID to a vector, so if we pick 
                //card 0 it sends the id of card 0 instead
            }
        }
        //Same as in Judge, bots pick at random.
        if self.is_bot
        {
            return competitors[rand::thread_rng().gen_range(0..competitors.len())];
        }
        else //player input
        {
            loop 
            {
                println!("Vote for the best card:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");

                let _ = match input.trim().parse::<usize>() 
                {
                    Ok(num) if num < *c_size => 
                    {
                        return competitors[num]; //return the id of the best player
                    },
                    _ => 
                    {
                        println!("Invalid input. Please try again.");
                        continue;
                    }
                };
            }
        }
    }
}

//probably the easiest way to create a new player
pub fn player_factory (id : i32, bot : bool, o: bool) -> Player 
{
    let p : Player = Player
    {
        player_id : id,
        is_bot : bot,
        //tbh I have no idea of why online is a thing, but it was in the og code so I'll let it be.
        online : o,
        hand : Vec::new(),
        green_apples : Vec::new(),
    };
    return p;
}
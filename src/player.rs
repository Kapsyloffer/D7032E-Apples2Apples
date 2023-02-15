#![allow(unused_imports)]
#![allow(dead_code)]
use crate::card::*;
use std::collections::HashMap;
use std::net::TcpStream;
use std::io::*;
use rand::Rng;
use std::io;
#[derive(Clone)]
pub struct Player //I guess all networking handlas av Client
{
    player_id : i32,
    is_bot : bool,
    online : bool,
    hand : Vec<RedCard>,
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
    fn vote(&self, cards : HashMap<i32, RedCard>) -> i32;//winner ID
}
pub trait Judge
{
   fn pick(&self, cards : &mut HashMap<i32, RedCard>) -> i32;//winner ID
}

impl Judge for Player //TODO: TEST
{
    #[allow(unused_variables)]
    fn pick(&self, cards : &mut HashMap<i32, RedCard>) -> i32
    {
        let mut competitors : Vec<i32> = Vec::new();
        let c_size = &cards.len();
        for c in cards
        {
            println!("{}:\n{}\n {}", competitors.len().to_string(), c.1.get_title(), c.1.get_desc());
            competitors.push(*c.0);
        }
        if self.is_bot
        {
            return competitors[rand::thread_rng().gen_range(0..competitors.len())];
        }
        else
        {
            loop 
            {
                println!("YOU ARE THE JUDGE\nPick the best card: ");
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
    fn add_to_hand(&mut self, rc : RedCard) 
    {
        self.hand.push(rc);
    }

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

    fn play_card (&mut self) -> RedCard  //TODO: TEST
    {
        //if bot, do random, else pick
        if self.is_bot
        {
            return self.hand.remove(rand::thread_rng().gen_range(0..self.hand.len()));
        }
        else
        {
            let mut i = 0;
            for c in self.hand.iter_mut()
            {
                println!("{}:  {} - {}", i.to_string(), c.get_title(), c.get_desc());
                i+=1;
            }
            loop 
            {
                println!("Pick your card:");
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

    fn vote(&self, cards : HashMap<i32, RedCard>) -> i32 //TODO: TEST
    {
        let mut competitors : Vec<i32> = Vec::new();
        let c_size = &cards.len();
        
        for c in &cards
        {
            if *c.0 != self.get_id()
            {
                println!("{}:\n{}\n {}", competitors.len().to_string(), c.1.get_title(), c.1.get_desc());
                competitors.push(*c.0);
            }
        }
        if self.is_bot
        {
            return competitors[rand::thread_rng().gen_range(0..competitors.len())];
        }
        else
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

pub fn player_factory (id : i32, bot : bool, o: bool) -> Player
{
    let p : Player = Player
    {
        player_id : id,
        is_bot : bot,
        online : o,
        hand : Vec::new(),
        green_apples : Vec::new(),
    };
    return p;
}
#![allow(unused_imports)]
#![allow(dead_code)]
use crate::card::*;
use crate::cardpiles::Discard;
use std::collections::HashMap;
use std::net::TcpStream;
use std::io::*;
use rand::Rng;
use rand::rngs::ThreadRng;
use std::io;

extern crate colorize;
use colorize::*;

#[derive(Clone)]
pub struct Player //I guess all networking handlas av Client
{
    player_id : i32,
    is_bot : bool,
    //online : bool,
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
    fn vote(&self, cards : &mut Vec<(i32, RedCard)>, cur_green : &GreenCard) -> i32; //returns the selected ID
    fn prompt_discard(&mut self, d : &mut Discard);
    fn prompt_wild_apple(&self) -> String;
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
            println!("\n=== {} ===", "YOUR HAND".yellow().bold());
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
                        if self.hand[num].is_wild()
                        {//This is very coupled :/
                            let wild_title = self.prompt_wild_apple();
                            self.hand[num].set_title(wild_title);
                        }
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

    fn vote(&self, cards : &mut Vec<(i32, RedCard)>, cur_green : &GreenCard) -> i32 //TODO: TEST
    {
        let mut competitors : Vec<i32> = Vec::new();
        let c_size = &cards.len();

        if !self.is_bot 
        {
        //CLEAR SCREEN
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        //Print current green card, makes voting easier.
        println!("\n{}\n{}\n", &cur_green.get_title().green().bold(), &cur_green.get_desc().green());        
        }
        //print out each card not played by the player (random ofc)
        for c in cards
        {
            if c.0 != self.get_id()
            {
                if !self.is_bot
                {
                    println!("{}: {}\n{}", competitors.len().to_string().yellow(), c.1.get_title().red().bold(), c.1.get_desc().red());
                }
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
                println!("\n==== {} ====", "VOTING PHASE".cyan().bold());
                println!("Vote for the best card:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");

                let _ = match input.trim().parse::<usize>() 
                {
                    Ok(num) if num < *c_size => 
                    { 
                        println!("You chose {}", competitors[num].to_string());
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

    fn prompt_discard(&mut self, discard_deck : &mut Discard) 
    {
        if self.is_bot
        {
            //The AI is not too complicated, choose a random amount of cards and discard em.
            let r_index = rand::thread_rng().gen_range(0..self.get_hand_size());
            discard_deck.add_to_discard(self.hand.remove(r_index as usize));
        }
        else 
        {
            //CLEAR SCREEN
            //print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    
            loop 
            {
                //Make a hashmap and count the loops.
                let mut c_hashmap : HashMap<i32, RedCard> = HashMap::new();
                let mut i = 0;
                //Print out each card in hand
                for c in self.hand.iter_mut()
                {
                    println!("{}: {}\n -{}", i.to_string().yellow(), c.get_title().red().bold(), c.get_desc().red());
                    //Add the cards to the hashmap
                    c_hashmap.insert(i, c.clone());
                    i+=1;
                }

                //Ask for which cards to discard.
                let mut input = String::new();
                
                println!("==== {} ====", "DISCARD PHASE".cyan().bold());
                println!("Enter the cards you wish to discard (leave empty to skip):");
                match io::stdin().read_line(&mut input) 
                {
                    Ok(_) => 
                    {
                        // Successfully read input
                        if input.trim().is_empty() 
                        {
                            break;
                        } 
                        else 
                        {
                            let numbers: Vec<i32> = input.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();

                            let mut new_hand: Vec<RedCard> = Vec::new();

                            //Get the discarded card index in the hashmap and remove them
                            for n in numbers
                            {
                                match c_hashmap.get(&n)
                                {
                                    Some(c) => 
                                    {
                                        println!("Discarded: {}", c.get_title().red());
                                        //Skicka kortet straight to discard.
                                        discard_deck.add_to_discard(c_hashmap.remove(&n).unwrap());
                                    }
                                    None => 
                                    {
                                        println!("Found no card of index {}", n)
                                    }
                                }
                            }

                            //Push the non-removed cards to new_hand.
                            for c in c_hashmap
                            {
                                new_hand.push(c.1);
                            }

                            //replace the old hand.
                            self.hand = new_hand;
                            break;
                            //fill hand handled in game.     
                        };
                    },
                    Err(error) => 
                    {
                        // Error reading input
                        eprintln!("Error reading input: {}", error.to_string());
                    }
                }
            }
        }
    }

    fn prompt_wild_apple(&self) -> String
    {
        if self.is_bot
        {
            //I don't know how I wish to deal with the bots in this.
            return "The funniest card ever".to_string();
        }
        else
        {
            loop 
            {
                println!("{}", "\n==WILD RED APPLE==\n".yellow().bold());
                println!("=== {} ===", "Write something funny".cyan());
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");

                let _ = match input.trim().parse::<String>() 
                {
                    Ok(red) =>
                    {
                        println!("YOU WROTE: {}", red);
                        return red;
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
pub fn player_factory (id : i32, bot : bool, _o: bool) -> Player 
{
    let p : Player = Player
    {
        player_id : id,
        is_bot : bot,
        //tbh I have no idea of why online is a thing, but it was in the og code so I'll let it be.
        //online : o,
        hand : Vec::new(),
        green_apples : Vec::new(),
    };
    return p;
}
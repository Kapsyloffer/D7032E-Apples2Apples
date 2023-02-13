#![allow(unused_imports)]
#![allow(dead_code)]
use crate::card::*;
use std::collections::HashMap;
use std::net::TcpStream;
use std::io::*;
#[derive(Clone)]
pub struct Player
{
    player_id : i32,
    is_bot : bool,
    online : bool,
    //connection: Option<TcpStream>,
    //in_from_client: Option<BufReader<TcpStream>>,
    //out_to_client: Option<BufWriter<TcpStream>>,
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
    //fn vote(&self, cards : HashMap<i32, RedCard>) -> i32;//winner ID
}
pub trait Judge
{
   fn pick(&self, cards : &mut HashMap<i32, RedCard>) -> i32;//winner ID
}

impl Player
{
    #[allow(dead_code)]
    fn do_stuff(&self)
    {
        todo!();
    }
}

impl Judge for Player
{
    #[allow(unused_variables)]
    fn pick(&self, cards : &mut HashMap<i32, RedCard>) -> i32
    {
        todo!();
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

    fn play_card (&mut self) -> RedCard 
    {
        //if bot, do random, else pick
        todo!()
        //foreach card in hand, print: THESE ARE YOUR CARDS; 1. {title} {desc}, 2. ...
        //ask for input,
    }
}

pub fn player_factory (id : i32, bot : bool, o: bool) -> Player
{
    let p : Player = Player
    {
        player_id : id,
        is_bot : bot,
        online : o,
        //connection : socket,
        //inFromClient : BufferedReader,
        //outToClient : DataOutputStream,
        hand : Vec::new(),
        green_apples : Vec::new(),
    };
    return p;
}
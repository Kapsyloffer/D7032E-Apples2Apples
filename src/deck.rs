use crate::card::*;
use crate::player::Player;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use rand::Rng;

pub trait Setup
{
    fn read_cards (&mut self) -> io::Result<String>;
    fn shuffle (&mut self);
}

#[allow(dead_code)]
pub struct RedDeck
{
    pub cards : Vec<RedCard>
}

#[allow(dead_code)]
pub struct GreenDeck
{
    pub cards : Vec<GreenCard>
}

pub struct Discard
{
    pub cards : Vec<RedCard>
}

impl Setup for GreenDeck
{
    fn read_cards (&mut self) -> io::Result<String>
    { 
        let mut new_deck : Vec<GreenCard> = Vec::new();
        let file = File::open("txt_files/greenApples.txt")?;
        let reader = BufReader::new(file);

        for line in reader.lines()
        {
            //let mut title_and_desc : Vec![&str] = line.split("-");
            let new_green = GreenCard
            {
                title: line.unwrap(),
                desc: "pending".to_string()
            };
            new_deck.push(new_green);
        }
        //Reset deck and add the new cards 
        self.cards.clear();
        self.cards = new_deck;

        Ok("Green Deck made".to_string())
    }

    fn shuffle (&mut self)
    {
        //Fisher Yates shuffle algorithm
        let mut deck : Vec<GreenCard> = self.cards.clone();
        let size : u8 = self.cards.len() as u8;

        for i in 0..size
        {   
            //Select last element
            let j : GreenCard = deck.pop().unwrap();
            //rnd [0 -> size-i]
            let rnd : u8 = rand::thread_rng().gen_range(0..(size-i));
            //Switch element[size] with element[size-i]
            let k : GreenCard = deck[usize::from(rnd)].clone();
            deck[usize::from(rnd)] = j;
            deck.push(k);
        }
        //Set the current deck to the shuffled deck.
        self.cards = deck;
    }
}

impl Setup for RedDeck
{
    fn read_cards (&mut self) -> io::Result<String>
    {
        let mut new_deck : Vec<RedCard> = Vec::new();
        let file = File::open("txt_files/redApples.txt")?;
        let reader = BufReader::new(file);
        for line in reader.lines()
        {
            //let mut title_and_desc : Vec![&str] = line.split("-");
            let new_red = RedCard
            {
                title: line.unwrap(),
                desc: "pending".to_string()
            };
            new_deck.push(new_red);
        }

        //Reset deck and add the new cards 
        self.cards.clear();
        self.cards = new_deck;

        println!("Red deck ready, SIZE: {}", self.cards.len());

        Ok("Red Deck made".to_string())
    }

    fn shuffle (&mut self)  
    {
        //Fisher Yates shuffle algorithm
        let mut deck : Vec<RedCard> = self.cards.clone();
        let size : u8 = self.cards.len() as u8;

        for i in 0..size
        {   
            //Select last element
            let j : RedCard = deck.pop().unwrap();
            //rnd [0 -> size-i]
            let rnd : u8 = rand::thread_rng().gen_range(0..(size-i));
            //Switch element[size] with element[size-i]
            let k : RedCard = deck[usize::from(rnd)].clone();
            deck[usize::from(rnd)] = j;
            deck.push(k);
        }
        //Set the current deck to the shuffled deck.
        self.cards = deck;
    }
}

impl RedDeck
{
    fn _deal (&mut self, player_list : Vec<Player>)
    {
        //Access each players hand
        for _p in player_list
        {
            //View size of hand
            
            //give n cards where n = 7-x (x is cards in hand)
            //Do the same for the next player in the list.
        }
        todo!()
    }
}
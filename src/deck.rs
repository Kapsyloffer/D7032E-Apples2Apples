use crate::card::*;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use rand::Rng;

pub trait Setup
{
    fn read_cards (&mut self) -> io::Result<String>;
    fn shuffle (&mut self);
}

#[derive(Clone)]
pub struct RedDeck
{
    pub cards : Vec<RedCard>
}

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
            if let Ok(line_str) = line 
            {
                let parts: Vec<&str> = line_str.splitn(2, "] -").collect();
                if parts.len() == 2 
                {
                    let new_green = GreenCard 
                    {
                        title: parts[0].to_string(),
                        desc: parts[1].to_string(),
                    };
                    new_deck.push(new_green);
                } 
                else 
                {
                    eprintln!("Skipping invalid line: {}", line_str);
                }
            } 
            else 
            {
                eprintln!("Error reading line: {:?}", line);
            }
        }
        //Reset deck and add the new cards 
        self.cards.clear();
        self.cards = new_deck;

        return Ok("Green Deck made".to_string());
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
        let file = File::open("txt_files/redApples.txt")?;
        let reader = BufReader::new(file);
        //Clear the deck so it doesn't duplicate
        self.cards.clear();
        for line in reader.lines() 
        {
            if let Ok(line_str) = line 
            {
                let parts: Vec<&str> = line_str.splitn(2, " -").collect();
                if parts.len() == 2 
                {
                    let new_red = RedCard 
                    {
                        title: parts[0].to_string(),
                        desc: parts[1].to_string(),
                    };
                    self.add_to_deck(new_red);
                } 
                else 
                {
                    eprintln!("Skipping invalid line: {}", line_str);
                }
            } 
            else 
            {
                eprintln!("Error reading line: {:?}", line);
            }
        }

        return Ok("Red Deck made".to_string());
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
    pub fn draw (&mut self) -> RedCard
    {
        let card = self.cards.remove(0);
        return card;
    }

    pub fn add_to_deck(&mut self, rc: RedCard)
    {
        self.cards.push(rc);
    }
}

impl Discard
{
    pub fn add_to_discard(&mut self, r : RedCard)
    {
        self.cards.push(r);
    }

    pub fn get_size(&self) -> usize
    {
        return self.cards.len();
    }
}
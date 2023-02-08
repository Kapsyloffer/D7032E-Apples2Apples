use crate::card::*;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub trait Setup
{
    fn read_cards (&mut self) -> io::Result<String>;
    fn shuffle (&mut self);
    fn deal (&mut self);
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
        todo!()
    }

    fn deal (&mut self)
    {
        todo!()
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

        Ok("Red Deck made".to_string())
    }

    fn shuffle (&mut self)
    {
        todo!()
    }

    fn deal (&mut self)
    {
        todo!()
    }
}

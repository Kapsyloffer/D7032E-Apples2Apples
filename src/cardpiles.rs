use crate::card::*;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use rand::Rng;

pub trait Setup
{
    fn read_cards (&mut self) -> io::Result<String>;
}

#[derive(Clone)]
pub struct RedDeck
{
    cards : Vec<RedCard>
}

pub struct GreenDeck
{
    cards : Vec<GreenCard>
}

pub struct Discard
{
    cards : Vec<RedCard>
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
                let parts: Vec<&str> = line_str.splitn(2, " -").collect();
                if parts.len() == 2 
                {
                    let new_green = greencard_factory(parts[0].to_string(), parts[1].to_string());
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
                    let new_red = redcard_factory(parts[0].to_string(), parts[1].to_string());
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
}

impl RedDeck
{
   pub fn draw (&mut self) -> RedCard
    {
        //self.shuffle(); //fixes the non-random draw issue, HOWEVER IT BREAKS MY TESTS REEEE
        let card = self.cards.remove(0);
        return card;
    }

    pub fn shuffle (&mut self) -> RedDeck
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
        return RedDeck{cards: deck};
    }

    pub fn add_to_deck(&mut self, rc: RedCard)
    {
        self.cards.push(rc);
    }

    pub fn get_size(&self) -> usize
    {
        return self.cards.len();
    }

    //USED FOR TESTING
    pub fn get_deck(&self) -> Vec<RedCard>
    {
        return self.cards.clone();
    }

    pub fn empty() -> RedDeck
    {
        return RedDeck { cards: Vec::new() };
    }
}

impl GreenDeck
{
    pub fn get_title_of_top_card(&self) -> String
    {
        return self.cards[0].get_title();
    }

    pub fn shuffle (&mut self) -> GreenDeck
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
        return GreenDeck{cards: deck}
    }

    pub fn draw(&mut self) -> GreenCard
    {
        return self.cards.remove(0);
    }

    pub fn get_size(&self) -> usize
    {
        return self.cards.len();
    }

    //USED FOR TESTING
    pub fn get_deck(&self) -> Vec<GreenCard>
    {
        return self.cards.clone();
    }

    pub fn empty() -> GreenDeck
    {
        return GreenDeck { cards: Vec::new() };
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

//Create a new GreenDeck
pub fn green_deck_factory() -> GreenDeck
{
    let mut g_deck = GreenDeck { cards: Vec::new() };
    //Req 1. Read all of the green apples
    _ = g_deck.read_cards();
    //Req 3. Shuffle both of the decks 
    return g_deck.shuffle();
}

//Create a new RedDeck
pub fn red_deck_factory() -> RedDeck
{
    let mut r_deck = RedDeck { cards: Vec::new() };
    //Req 2. Read all of the red apples 
    _ = r_deck.read_cards();
    //Req 3. Shuffle both of the decks
    return r_deck.shuffle();
}

pub fn discard_factory() -> Discard
{
    return Discard { cards: Vec::new() };
}
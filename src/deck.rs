use crate::card::*;

trait Setup
{
    fn read_cards (&self);
    fn shuffle (&self);
    fn deal (&self);
}

#[allow(dead_code)]
struct RedDeck
{
    cards : Vec<RedCard>
}

#[allow(dead_code)]
struct GreenDeck
{
    cards : Vec<GreenCard>
}

impl Setup for GreenDeck
{
    fn read_cards (&self) 
    {
        todo!()
    }

    fn shuffle (&self) 
    {
        todo!()
    }

    fn deal (&self) 
    {
        todo!()
    }
}

impl Setup for RedDeck
{
    fn read_cards (&self) 
    {
        todo!()
    }

    fn shuffle (&self) 
    {
        todo!()
    }

    fn deal (&self) 
    {
        todo!()
    }
}

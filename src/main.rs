use apples::card::*;
use apples::deck::*;
fn main() 
{
    

    let mut _rc: RedDeck = RedDeck
    {
        cards : Vec::new()
    };

    _rc.read_cards();

    println!("Red: {}\n", _rc.cards.len().to_string());
}
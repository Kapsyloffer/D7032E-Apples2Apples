//use apples::card::*;
use apples::deck::*;
use apples::game::*;
fn main() 
{
    let mut rc: RedDeck = RedDeck
    {
        cards : Vec::new()
    };

    let _q = rc.read_cards();

    println!("Red: {}\n", rc.cards.len().to_string());
    println!("Hello world!");
    init_game();
}
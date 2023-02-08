use crate::player;
use crate::deck::*;
use crate::card::*;
#[allow(dead_code)]
#[allow(unused_variables)]
pub fn player_factory (id : i32, is_bot : bool, online: bool) -> player::Player
{
    todo!()
}

#[allow(unreachable_code)]
fn game_setup()
{
    //Create all the decks
    let mut r_deck = RedDeck{cards: Vec::new()};
    let mut g_deck = GreenDeck{cards: Vec::new()};
    let mut d_deck = Discard{cards: Vec::new()}; //Discard
    //1. Read all of the green apples
    //2. Read all of the red apples 
    r_deck.read_cards();
    g_deck.read_cards();
    //TODO: Maybe create a deck.rs and read in there under a fn init?

    //3. Shuffle both of the decks 
    r_deck.shuffle();
    g_deck.shuffle();

    //4. Deal 7 red apples to each player
    //TODO: foreach player, add to playerlist, send playerlist into deck and do the deal.

    //5. Pick a judge at random.
    //TODO: player_list[rnd(1..size)] eller nåt
}

#[allow(dead_code)]
fn gameplay()
{
    //6. A green apple is drawn from the pile and shown to everyone
    //7. All players except the judge plays a red Apple
    //8. Order is randomized before shown.
    //9. All players must play a card before the results at 8 are shown.
    //10. Judge picks card, winner gets the green apple.
    //11. All red apples end up in the discard pile.
    //12. All players draw 7-n cards where n is their handsize
    //13. Next player in the list becomes judge.

    /*Here’s how to tell when the game is over:
    • For 4 players, 8 green apples win.
    • For 5 players, 7 green apples win.
    • For 6 players, 6 green apples win.
    • For 7 players, 5 green apples win.
    • For 8+ players, 4 green apples win.*/
}
#[allow(unused_imports)]
use crate::game::*;
use crate::player::*;
use crate::card::*;
use crate::deck::*;    
use rand::Rng;
use core::hash::*;
use std::hash::Hash;

#[cfg(test)]
//SETUP
#[test]
fn read_all_green_apples() //Req 1
{
    let mut _gc : GreenDeck = GreenDeck
    {
        cards : Vec::new()
    };
    let b4 : i32 = _gc.cards.len() as i32;
    let _c = _gc.read_cards();
    let after : i32 = _gc.cards.len() as i32;
    assert_ne!(b4, after);
}
#[test]
fn read_all_red_apples()  //Req 2
{
    let mut _rc : RedDeck = RedDeck
    {
        cards : Vec::new()
    };
    let b4 : i32 = _rc.cards.len() as i32;
    let _c = _rc.read_cards();
    let after : i32 = _rc.cards.len() as i32;
    assert_ne!(b4, after);
}
#[test]
fn shuffle_both_decks()  //Req 3
{
    //Makes new instances of the decks
    let mut rd = RedDeck{ cards : Vec::new()};
    let mut gd= GreenDeck{ cards : Vec::new()};

    //Init all decks from files
    let _c = rd.read_cards();
    let _d = gd.read_cards();

    //Save state before shuffle
    let b4shuffle_r : Vec<RedCard> = rd.cards.clone();
    let b4shuffle_g : Vec<GreenCard> = gd.cards.clone();

    //Shuffle both
    rd.shuffle();
    gd.shuffle();

    //Check the difference
    //Sometimes it fails because of the random engine, idk either man.
    assert_ne!(hash_value(b4shuffle_r), hash_value(rd.cards));
    assert_ne!(hash_value(b4shuffle_g), hash_value(gd.cards));
}

//used for the test above.
fn hash_value<T: Hash>(deck: T) -> u64 
{
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    deck.hash(&mut hasher);
    hasher.finish()
}

#[test]
fn deal7_red_apples_to_each_player()  //Req 4
{
    //ny spelare 1 och 2 och 3, de har 2, 5, och 7 kort respectively.
    let mut p_list : Vec<Player> = Vec::new();
    //Create 4 players with empty hands
    for x in 1..4
    {
       let p : Player = player_factory(x, false, true);
       p_list.push(p);
    }

    let mut dummy_deck : RedDeck = RedDeck{ cards: Vec::new()};
    //Generate a dummy deck for refilling
    for _d in 0.. (p_list.len() * 7)
    {
        let dummy_card = RedCard{title: "Dummy card".to_string(), desc: "Fill card".to_string()};
        dummy_deck.add_to_deck(dummy_card);
    }

    for p in 0..p_list.len()
    {
        let mut dummy_p = p_list[p].clone();
        dummy_p = refill_hand(dummy_p, dummy_deck.clone());
        p_list.remove(0);
        p_list.push(dummy_p);
    }

    for p in 0..p_list.len()
    {
        println!("I, player: {}, have this many cards: {} \n", p.to_string(), p_list[p].get_hand_size().to_string());
        assert_eq!(p_list[p].get_hand_size(), 7);
    }
}

#[test]
fn pick_a_judge_at_random()  //Req 5
{
    let mut p_list : Vec<Player> = Vec::new();
    //Create 5 dummy players
    for x in 0..4
    {
        p_list.push(player_factory(x, false, true));
    }

    let mut prev_p = judge_pick(&p_list);
    let mut cur_p : Player;
    //Try 10 times, probability of it always being the same outta 5 is 0.00001024% (1/5 ^10)
    let mut dupes : i32 = 0;
    let mut loops : i32 = 0;
    for _ in 1..10
    {
        cur_p = judge_pick(&p_list);
        if cur_p.get_id() == prev_p.get_id()
        {
            dupes +=1;
        }
        prev_p = cur_p;
        loops = &loops +1;
    }
    assert_ne!(&dupes, &loops);
}

//GAMEPLAY

#[test]
fn green_apple_drawn_and_shown_to_everyone()  //Req 6
{
    assert_eq!(1, 0);
}

#[test]
fn judge_do_not_play_red_apple()  //Req 7
{
    assert_eq!(1, 0);
}

#[test]
fn order_of_cards_random_before_shown()  //Req 8
{
    assert_eq!(1, 0);
}

#[test]
fn all_players_must_play_before_result_is_shown()  //Req 9
{
    assert_eq!(1, 0);
}

#[test]
fn judge_picks_card_winner_gets_green_apple()  //Req 10
{
    assert_eq!(1, 0);
}

#[test]
fn all_red_apples_go_to_discard()  //Req 11
{
    assert_eq!(1, 0);
}

#[test]
fn all_players_draw_up_to_seven()  //Req 12 (Literally just Req4 but we start with cards in hand)
{
    let mut p_list : Vec<Player> = Vec::new();
    //Create 4 players
    for x in 1..4
    {
       let mut p : Player = player_factory(x, false, true);
       //give each player a random hand of 0 to 7 cards
        for _x in 1..rand::thread_rng().gen_range(1..7) 
        {
            let new_card = RedCard{title: "Test".to_string(), desc : "Testcard".to_string()};
            p.add_to_hand(new_card);
        }
        println!("I, player: {}, have this many cards: {} \n", x.to_string(), p.get_hand_size().to_string());
        p_list.push(p);
    }

    let mut dummy_deck : RedDeck = RedDeck{ cards: Vec::new()};
    //Generate a dummy deck for refilling
    for _d in 0.. (p_list.len() * 7)
    {
        let dummy_card = RedCard{title: "Dummy card".to_string(), desc: "Fill card".to_string()};
        dummy_deck.add_to_deck(dummy_card);
    }

    for p in 0..p_list.len()
    {
        let mut dummy_p = p_list[p].clone();
        dummy_p = refill_hand(dummy_p, dummy_deck.clone());
        p_list.remove(0);
        p_list.push(dummy_p);
    }

    for p in 0..p_list.len()
    {
        println!("I, player: {}, have this many cards: {} \n", p.to_string(), p_list[p].get_hand_size().to_string());
        assert_eq!(p_list[p].get_hand_size(), 7);
    }
}

#[test]
fn next_player_in_list_becomes_judge()  //Req 13
{
    //let mut p_list : Vec<Player> = Vec::new();
    //add like 11 boys
    //pick a Judge at random
    //Step 1
    //???
    //Pass
    assert_eq!(1, 0);
}

#[test]
fn check_for_winner_4p()  //Req 14
{
    assert_eq!(1, 0);
}

#[test]
fn check_for_winner_5p() 
{
    assert_eq!(1, 0);
}

#[test]
fn check_for_winner_6p() 
{
    assert_eq!(1, 0);
}

#[test]
fn check_for_winner_7p() 
{
    assert_eq!(1, 0);
}

#[test]
fn check_for_winner_8plus() 
{
    assert_eq!(1, 0);
}
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
    let mut _gc : GreenDeck = GreenDeck {cards : Vec::new()};
    let b4 = _gc.cards.len();
    let _ = _gc.read_cards();
    let after = _gc.cards.len();
    assert_ne!(b4, after);
}
#[test]
fn read_all_red_apples()  //Req 2
{
    let mut _rc : RedDeck = RedDeck {cards : Vec::new()};
    let b4  = _rc.cards.len();
    let _ = _rc.read_cards();
    let after = _rc.cards.len();
    assert_ne!(b4, after);
}
#[test]
fn shuffle_both_decks()  //Req 3
{
    //Makes new instances of the decks
    let mut rd = RedDeck{ cards : Vec::new()};
    let mut gd= GreenDeck{ cards : Vec::new()};

    //Init all decks from files
    let _ = rd.read_cards();
    let _ = gd.read_cards();

    //Save state before shuffle
    let b4shuffle_r : Vec<RedCard> = rd.cards.clone();
    let b4shuffle_g : Vec<GreenCard> = gd.cards.clone();

    //Shuffle both
    rd.shuffle();
    gd.shuffle();

    //Check the difference
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
       p_list.push(player_factory(x, false, true));
    }

    let mut dummy_deck : RedDeck = RedDeck{ cards: Vec::new()};
    //Generate a dummy deck for refilling
    for _d in 0.. (p_list.len() * 7)
    {
        let dummy_card = RedCard{title: "Dummy card".to_string(), desc: "Fill card".to_string()};
        dummy_deck.add_to_deck(dummy_card);
    }

    //Refills hand properly
    for p in 0..p_list.len()
    {
        refill_hand(&mut p_list[p], &mut dummy_deck);
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
        refill_hand(&mut p_list[p], &mut dummy_deck);
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
    let mut p_list : Vec<Player> = Vec::new();
    //add like 11 boys
    for x in 0..11
    {
        p_list.push(player_factory(x, false, true));
    }
    //???
    for _ in 1..100
    {
        //pick a Judge at random
        let judge = judge_pick(&p_list);
        //next judge
        let judge_next = next_judge(&p_list, &judge);
        
        if &judge_next.get_id() == &0
        {
            //Om vi har 14 spelare blir går id: 11, 12, 13, 0, 1, 2 ---
            assert_eq!(&judge.get_id() - (p_list.clone().len() as i32 -1), &judge_next.get_id()+0);
        }
        else
        {
            assert_eq!(&judge.get_id() + 1, &judge_next.get_id()+0);
        }
    }
    //Pass
}

#[test]
fn check_for_winner_4p()  //Req 14
{
    let mut p_list : Vec<Player> = Vec::new();
    //Generate 4 players
    for i in 0..4
    {
        p_list.push(player_factory(i, true, true));
    }
    //Check
    assert_ne!(true, check_winner(&p_list));
    //Create a dummy green deck
    let mut g_deck = GreenDeck{cards:Vec::new()};
    let _ = g_deck.read_cards();
    let _ = g_deck.shuffle();
    //give a player 8 green
    for _ in 0..8
    {
        println!("{}\n", &g_deck.cards[0].title);
        p_list[0].give_green(g_deck.cards.remove(0));
    }
    //check
    assert_eq!(true, check_winner(&p_list));
}

#[test]
fn check_for_winner_5p() 
{
    let mut p_list : Vec<Player> = Vec::new();
    //Generate 5 players
    for i in 0..5
    {
        p_list.push(player_factory(i, true, true));
    }
    //Check
    assert_ne!(true, check_winner(&p_list));
    //Create a dummy green deck
    let mut g_deck = GreenDeck{cards:Vec::new()};
    let _ = g_deck.read_cards();
    let _ = g_deck.shuffle();
    //give a player 7 green
    for _ in 0..7
    {
        println!("{}\n", &g_deck.cards[0].title);
        p_list[0].give_green(g_deck.cards.remove(0));
    }
    //check
    assert_eq!(true, check_winner(&p_list));
}

#[test]
fn check_for_winner_6p() 
{
    let mut p_list : Vec<Player> = Vec::new();
    //Generate 6 players
    for i in 0..6
    {
        p_list.push(player_factory(i, true, true));
    }
    //Check
    assert_ne!(true, check_winner(&p_list));
    //Create a dummy green deck
    let mut g_deck = GreenDeck{cards:Vec::new()};
    let _ = g_deck.read_cards();
    let _ = g_deck.shuffle();
    //give a player 6 green
    for _ in 0..6
    {
        println!("{}\n", &g_deck.cards[0].title);
        p_list[0].give_green(g_deck.cards.remove(0));
    }
    //check
    assert_eq!(true, check_winner(&p_list));
}

#[test]
fn check_for_winner_7p() 
{
    let mut p_list : Vec<Player> = Vec::new();
    //Generate 7 players
    for i in 0..7
    {
        p_list.push(player_factory(i, true, true));
    }
    //Check
    assert_ne!(true, check_winner(&p_list));
    //Create a dummy green deck
    let mut g_deck = GreenDeck{cards:Vec::new()};
    let _ = g_deck.read_cards();
    let _ = g_deck.shuffle();
    //give a player 5 green
    for _ in 0..5
    {
        println!("{}\n", &g_deck.cards[0].title);
        p_list[0].give_green(g_deck.cards.remove(0));
    }
    //check
    assert_eq!(true, check_winner(&p_list));
}

#[test]
fn check_for_winner_8() 
{
    let mut p_list : Vec<Player> = Vec::new();
    //Generate 8 players
    for i in 0..8
    {
        p_list.push(player_factory(i, true, true));
    }
    //Check
    assert_ne!(true, check_winner(&p_list));
    //Create a dummy green deck
    let mut g_deck = GreenDeck{cards:Vec::new()};
    let _ = g_deck.read_cards();
    let _ = g_deck.shuffle();
    //give a player 4 green
    for _ in 0..4
    {
        println!("{}\n", &g_deck.cards[0].title);
        p_list[0].give_green(g_deck.cards.remove(0));
    }
    //check
    assert_eq!(true, check_winner(&p_list));
}

#[test]
fn check_for_winner_8plus() 
{
    let mut p_list : Vec<Player> = Vec::new();
    //Generate 8 players
    for i in 0..54
    {
        p_list.push(player_factory(i, true, true));
    }
    //Check
    assert_ne!(true, check_winner(&p_list));
    //Create a dummy green deck
    let mut g_deck = GreenDeck{cards:Vec::new()};
    let _ = g_deck.read_cards();
    let _ = g_deck.shuffle();
    //give a player 4 green
    for _ in 0..8
    {
        println!("{}\n", &g_deck.cards[0].title);
        p_list[0].give_green(g_deck.cards.remove(0));
    }
    //check
    assert_eq!(true, check_winner(&p_list));
}
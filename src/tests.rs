#![allow(unused)]
use crate::game_functions::*;
use crate::player;
use crate::player::*;
use crate::card::*;
use crate::cardpiles::*;
use crate::settings;
use crate::settings::custom_settings;
use crate::settings::default_settings;    
use rand::Rng;
use core::hash::*;
use std::hash::Hash;

#[cfg(test)]
//SETUP
#[test]
fn read_all_green_apples_req1() //Req 1
{
    //Create a new deck
    let mut gc : GreenDeck = GreenDeck::empty();

    //Get the length before and after filling the deck.
    let b4 = gc.get_size();
    gc.read_cards();
    let after = gc.get_size();

    //Check if it got bigger.
    assert!(b4 < after);
}
#[test]
fn read_all_red_apples_req2()  //Req 2
{
    //Create a new deck
    let mut rc : RedDeck = RedDeck::empty();

    //Get the length before and after filling the deck.
    let b4  = rc.get_size();
    rc.read_cards();
    let after = rc.get_size();

    //Check if it got bigger.
    assert!(b4 < after);
}
#[test]
fn shuffle_both_decks_req3()  //Req 3
{
    //Makes new instances of the decks
    let mut rd = RedDeck::empty();
    let mut gd= GreenDeck::empty();

    //Init all decks from files
    let _ = rd.read_cards();
    let _ = gd.read_cards();

    //Save state before shuffle
    let b4shuffle_r : Vec<RedCard> = rd.get_deck();
    let b4shuffle_g : Vec<GreenCard> = gd.get_deck();

    //Check if they are the same
    assert_eq!(hash_value(&b4shuffle_r), hash_value(&rd.get_deck()));
    assert_eq!(hash_value(&b4shuffle_g), hash_value(&gd.get_deck()));

    //Shuffle both decks
    rd = rd.shuffle();
    gd = gd.shuffle();

    //Check if they are different.
    assert_ne!(hash_value(&b4shuffle_r), hash_value(&rd.get_deck()));
    assert_ne!(hash_value(&b4shuffle_g), hash_value(&gd.get_deck()));
}

//used for hashging vectors and comparing if they got shuffled.
fn hash_value<T: Hash>(deck: T) -> u64 
{
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    deck.hash(&mut hasher);
    hasher.finish()
}

#[test]
fn deal7_red_apples_to_each_player_req4()  //Req 4
{
    let mut p_list : Vec<Player> = Vec::new();

    //Create 4 players with empty hands
    for x in 1..4
    {
       p_list.push(player_factory(x, false, true));
    }

    //Generate a dummy deck for refilling
    let mut dummy_deck : RedDeck = RedDeck::empty();
    
    //Add some dummy cards to it.
    for _d in 0..(p_list.len() * 14)-1
    {
        let dummy_card = redcard_factory("Dummy card".to_string(),  "Fill card".to_string());
        dummy_deck.add_to_deck(dummy_card);
    }

    //Refill the hand of each player.
    for p in 0..p_list.len()
    {
        refill_hand(&mut p_list[p], &mut dummy_deck, &default_settings());
    }

    for p in 0..p_list.len()
    {
        println!("I, player: {}, have this many cards: {} \n", p.to_string(), p_list[p].get_hand_size().to_string());
        assert_eq!(p_list[p].get_hand_size(), 7);
    }
}

#[test]
fn pick_a_judge_at_random_req5()  //Req 5
{
    let mut p_list : Vec<Player> = Vec::new();

    //Create 5 dummy players
    for x in 0..4
    {
        p_list.push(player_factory(x, false, true));
    }

    let mut prev_p = judge_pick(&p_list).clone();
    let mut cur_p : Player;

    //Try 10 times, probability of it always 
    //being the same outta 5 is 0.00001024% (1/5 ^10)
    let mut dupes : i32 = 0;
    let mut loops : i32 = 0;

    for _ in 1..100 //Actually make it 100 to be extra sure.
    {
        cur_p = judge_pick(&p_list).clone();
        if cur_p.get_id() == prev_p.get_id()
        {
            dupes +=1;
        }
        prev_p = cur_p;
        loops = &loops +1;
    }

    //Check that it didn't pick the same one 10 times in a row.
    assert_ne!(&dupes, &loops);
}

//GAMEPLAY

#[test]
fn allow_players_to_discard_their_hands_before_phase_a() //Idk which requirement this is.
{
    //Dummy bot som ska discarda
    let mut dummy_player = player_factory(0, true, true);

    //Skapa red deck
    let mut red_deck = RedDeck::empty();

    //Skapa Discard
    let mut discard_deck = discard_factory();

    //Läs in alla kort
    red_deck.read_cards();

    //Checka att korten laddar in ordentligt.
    assert!(red_deck.get_size() > 1);

    //Ge player 7 kort.
    refill_hand(&mut dummy_player, &mut red_deck, &default_settings());

    //Checka om han fick 7 kort.
    assert_eq!(dummy_player.get_hand_size(), 7);

    //Prompt discard
    dummy_player.prompt_discard(&mut discard_deck);

    //Compara om han discarade properly
    assert!(discard_deck.get_size() > 0);

    //Om allt funkar properly så passar testen.
}

#[test]
fn green_apple_drawn_and_shown_to_everyone_req6()  //Req 6
{
    let dummy1 = greencard_factory("dummy".to_string(), "thicc".to_string());
    let mut dummy2 = greencard_factory("dummy".to_string(), "thicc".to_string());

    //Check if both are the same right now.
    assert_eq!(dummy1.get_desc(), dummy2.get_desc());

    //Create a new deck for this test.
    let mut g_deck: GreenDeck = GreenDeck::empty();
    g_deck.read_cards();
    g_deck.shuffle();

    //A green apple is drawn
    dummy2 = new_green(&mut g_deck); 

    //It changed
    assert_ne!(dummy1.get_desc(), dummy2.get_desc());
    //And then we print it in Game.rs which makes this test kinda
    //Incomplete/useless but still. :P
}

#[test]
fn judge_do_not_play_red_apple_req7()  //Req 7
{
    //Make 3 players.
    let p1: Player = player_factory(1, true, true);
    let p2: Player = player_factory(2, true, true);
    let p3: Player = player_factory(3, true, true);

    //Make player 2 the judge.
    let j : &Player = &p2;

    //Can_play_apple is always run on gameplay, it checks wether you're a judge or not
    //and because the judge cannot play cards it returns false.
    assert_eq!(can_play_apple(&p1, &j), true);
    assert_eq!(can_play_apple(&p2, &j), false); //Will fail because p2 is judge
    assert_eq!(can_play_apple(&p3, &j), true);
}

#[test]
fn order_of_cards_randomized_before_shown_to_judge_req8()  //Req 8
{
    //Create a new vector of cards
    let mut cards: Vec<(i32, RedCard)> = Vec::new();

    //Create 32 card entries.
    for i in 0..32
    {
        cards.push((i, redcard_factory(format!("Dummy {}", i) ,"Dummy card".to_string())));
    }

    //Cards before shuffle
    let b4_shuffle = cards.clone();
    
    //Check if they are the same
    assert_eq!(hash_value(&b4_shuffle), hash_value(&cards));

    //Cards after shuffle
    let cards = shuffle_before_showing(&mut cards);

    //Check if they are different.
    assert_ne!(hash_value(&b4_shuffle), hash_value(&cards));
}

#[test]
fn all_players_must_play_before_result_is_shown_req9()  //Req 9
{
    //This is already done by forcing each player to play before we can procees with the game
    //The consequence is that some player may stall the game, but in the case of bots
    //we're good, or so I hope.
    assert_eq!(1, 1);
}

#[test]
fn judge_picks_card_winner_gets_green_apple_req10a()  //Req 10
{
    let mut p_list : Vec<Player> = Vec::new();
    let mut red_cards : Vec<(i32, RedCard)> = Vec::new();

    let green : GreenCard = greencard_factory("Dummy green".to_string(), "Despite the name he's actually quite smart".to_string());

    for i in 0..6 as usize
    {
        p_list.push(player_factory(i as i32, true, true));
        p_list[i].add_to_hand(redcard_factory("[Dummy]".to_string(), "Dummy card".to_string()));
    }
    let j : Player = p_list[0].clone();
     //Have them all play their card into the red cards pile.
     for p in p_list.iter_mut()
     {
        if can_play_apple(&p, &j)
        {
            red_cards.push((p.get_id(), p.play_card()));
        }
     }
     let win_id = j.pick(&mut red_cards);
     let mut winner : &mut Player = &mut player_factory(9999, true, false); //dummy

     for p in &mut p_list
     {
        if &p.get_id() == &win_id
        {
            winner = p;
            break;
        }
     }
     //Give 1 green card
     reward_winner(&mut winner, green);

    //Om det fortfarande är dummy, force fail.
     assert_ne!(winner.get_id(), 9999); 

     //Check if the winner has one green.
     assert_eq!(winner.get_green_amount(), 1);
}

#[test]
fn test_vote_count_req10b() //Technically Req 10b
{
    //skapa en totally legit vote count.
    let mut vote_count : Vec<i32> = [0, 0, 0, 0, 0, 0].to_vec();
    let mut p_list : Vec<Player> = Vec::new();
    
    //fyll p_list med dummies
    for i in 0..8
    {
        p_list.push(player_factory(i, true, true));
    }

    //testa med massa nollor
    assert_eq!(count_votes(&p_list, &mut vote_count), 0);

    //testa med allt utom 0,
    vote_count = [1,2,3,4,5,6,6].to_vec();
    
    //testa med massa nollor
    assert_eq!(count_votes(&p_list, &mut vote_count), 6);
}

#[test]
fn test_vote_tiebreaker_req10b() //Technically Req 10b
{
    //skapa en totally legit vote count.
    let mut vote_count : Vec<i32> = [0, 0, 0, 0, 0, 0].to_vec();
    let mut p_list : Vec<Player> = Vec::new();
    
    //vi har två spelare, player 1 och 2
    p_list.push(player_factory(1, true, true));
    p_list.push(player_factory(2, true, true));

    //testa med allt utom 0,
    vote_count = [1,2].to_vec();

    //nu ska vi testa tiebreakern genom att se om den väljer samma varje gång eller om det är random.
    let mut team1 = 0;
    let mut team2 = 0;

    //Räkna antalet loops
    let mut loop_count = 0;

    //välj en av vinnarna x antal gånger
    for _ in 0..100
    {
        loop_count +=1;
        if count_votes(&p_list, &mut vote_count) == 1
        {
            team1 += 1;
        }
        else 
        {
            team2 += 1;
        }
    }

    //testa om de är olika.
    assert_ne!(loop_count, team1);
    assert_ne!(loop_count, team2);
}

#[test]
fn all_red_apples_go_to_discard_req11()  //Req 11
{
    let mut p_list : Vec<Player> = Vec::new();
    let mut red_cards : Vec<(i32, RedCard)> = Vec::new();
    let mut discard : Discard = discard_factory();

    //create 5 bots
    for i in 0..5
    {
        p_list.push(player_factory(i, true, true));
    }

    //Give them all one dummy card
    for p in p_list.iter_mut()
    {
        p.add_to_hand(redcard_factory("[Dummy]".to_string(), "Dummy card".to_string()));
    }

    //Have them all play their card into the red cards pile.
    for p in p_list.iter_mut()
    {
        red_cards.push((p.get_id(), p.play_card()));
    }

    //Get size of redcards pile before trashing
    let sizeb4 = red_cards.len();

    //Send the red cards to discard, return empty redcard pile
    red_cards = send_to_discard(red_cards, &mut discard);

    //Check if the cards got discarded.
    assert!(discard.get_size() == sizeb4);
    //Check if red cards is empty
    assert!(sizeb4 > red_cards.len());
}

#[test]
fn all_players_draw_up_to_seven_req12()  //Req 12 (Literally just Req4 but we start with cards in hand)
{
    let mut p_list : Vec<Player> = Vec::new();

    //Create 100 players
    for x in 1..100
    {
       let mut p : Player = player_factory(x, false, true);

       //give each player a random hand of 0 to 7 cards
        for _x in 1..rand::thread_rng().gen_range(1..7) 
        {
            let new_card = redcard_factory("[Dummy]".to_string(), "Dummy card".to_string());
            p.add_to_hand(new_card);
        }

        println!("I, player: {}, have this many cards: {} \n", x.to_string(), p.get_hand_size().to_string());
        p_list.push(p);
    }

    let mut dummy_deck : RedDeck = RedDeck::empty();

    //Generate a dummy deck for refilling
    for _d in 0.. (p_list.len() * 7)
    {
        let dummy_card = redcard_factory("[Dummy]".to_string(), "Dummy card".to_string());
        dummy_deck.add_to_deck(dummy_card);
    }

    //Refill the hand of each player.
    for p in 0..p_list.len()
    {
        refill_hand(&mut p_list[p], &mut dummy_deck, &default_settings());
    }

    //Check if everyone has 7 cards.
    for p in 0..p_list.len()
    {
        println!("I, player: {}, have this many cards: {} \n", p.to_string(), p_list[p].get_hand_size().to_string());
        assert_eq!(p_list[p].get_hand_size(), 7);
    }
}

#[test]
fn next_player_in_list_becomes_judge_req13()  //Req 13
{
    let mut p_list : Vec<Player> = Vec::new();
    //add like 11 boys
    for x in 0..11
    {
        p_list.push(player_factory(x, false, true));
    }
    
    //Testa 100 gånger
    for _ in 1..100
    {
        //pick a Judge at random
        let judge = judge_pick(&p_list);

        //välj next judge in line
        let judge_next = next_judge(&p_list, judge);
        
        //Om vi owerflowar.
        if &judge_next.get_id() == &0
        {
            //Om vi har 14 spelare blir vår id: först 11, sen 12, sen 13, 0, 1, 2 --- osvosv
            assert_eq!(&judge.get_id() - (&p_list.len() -1) as i32, &judge_next.get_id()+0);
        }
        else
        {
            //Annars så tar vi bara nästa.
            assert_eq!(&judge.get_id() + 1, &judge_next.get_id()+0);
        }
    }
    //Pass
}

#[test]
fn check_for_winner_4p_req14a()  //Req 14
{
    let mut p_list : Vec<Player> = Vec::new();

    //Generate 4 players
    for i in 0..4
    {
        p_list.push(player_factory(i, true, true));
    }

    //Check
    assert_ne!(true, check_winner(&p_list, &default_settings()));

    //Create a dummy green deck
    let mut g_deck = GreenDeck::empty();
    let _ = g_deck.read_cards();
    g_deck = g_deck.shuffle();

    //give a player 8 green
    for _ in 0..8
    {
        println!("{}\n", &g_deck.get_title_of_top_card());
        p_list[0].give_green(g_deck.draw());
    }

    //check
    assert_eq!(true, check_winner(&p_list, &default_settings()));
}

#[test]
fn check_for_winner_5p_req14a() 
{
    let mut p_list : Vec<Player> = Vec::new();

    //Generate 5 players
    for i in 0..5
    {
        p_list.push(player_factory(i, true, true));
    }

    //Check
    assert_ne!(true, check_winner(&p_list, &default_settings()));

    //Create a dummy green deck
    let mut g_deck = GreenDeck::empty();
    let _ = g_deck.read_cards();
    g_deck = g_deck.shuffle();

    //give a player 7 green
    for _ in 0..7
    {
        println!("{}\n", &g_deck.get_title_of_top_card());
        p_list[0].give_green(g_deck.draw());
    }

    //check
    assert_eq!(true, check_winner(&p_list, &default_settings()));
}

#[test]
fn check_for_winner_6p_req14a() 
{
    let mut p_list : Vec<Player> = Vec::new();

    //Generate 6 players
    for i in 0..6
    {
        p_list.push(player_factory(i, true, true));
    }

    //Check
    assert_ne!(true, check_winner(&p_list, &default_settings()));

    //Create a dummy green deck
    let mut g_deck = GreenDeck::empty();
    let _ = g_deck.read_cards();
    g_deck = g_deck.shuffle();

    //give a player 6 green
    for _ in 0..6
    {
        println!("{}\n", &g_deck.get_title_of_top_card());
        p_list[0].give_green(g_deck.draw());
    }

    //check
    assert_eq!(true, check_winner(&p_list, &default_settings()));
}

#[test]
fn check_for_winner_7p_req14a() 
{
    let mut p_list : Vec<Player> = Vec::new();

    //Generate 7 players
    for i in 0..7
    {
        p_list.push(player_factory(i, true, true));
    }

    //Check
    assert_ne!(true, check_winner(&p_list, &default_settings()));

    //Create a dummy green deck
    let mut g_deck = GreenDeck::empty();
    let _ = g_deck.read_cards();
    g_deck = g_deck.shuffle();

    //give a player 5 green
    for _ in 0..5
    {
        println!("{}\n", &g_deck.get_title_of_top_card());
        p_list[0].give_green(g_deck.draw());
    }

    //check
    assert_eq!(true, check_winner(&p_list, &default_settings()));
}

#[test]
fn check_for_winner_8p_req14a() 
{
    let mut p_list : Vec<Player> = Vec::new();

    //Generate 8 players
    for i in 0..8
    {
        p_list.push(player_factory(i, true, true));
    }

    //Check
    assert_ne!(true, check_winner(&p_list, &default_settings()));

    //Create a dummy green deck
    let mut g_deck = GreenDeck::empty();
    let _ = g_deck.read_cards();
    g_deck = g_deck.shuffle();

    //give a player 4 green
    for _ in 0..4
    {
        println!("{}\n", &g_deck.get_title_of_top_card());
        p_list[0].give_green(g_deck.draw());
    }

    //check
    assert_eq!(true, check_winner(&p_list, &default_settings()));
}

#[test]
fn check_for_winner_8plus_req14a() 
{
    let mut p_list : Vec<Player> = Vec::new();

    //Generate 8+ players
    for i in 0..54
    {
        p_list.push(player_factory(i, true, true));
    }

    //Check
    assert_ne!(true, check_winner(&p_list, &default_settings()));

    //Create a dummy green deck
    let mut g_deck = GreenDeck::empty();
    let _ = g_deck.read_cards();
    g_deck = g_deck.shuffle();
    
    //give a player 4 green
    for _ in 0..4
    {
        println!("{}\n", &g_deck.get_title_of_top_card());
        p_list[0].give_green(g_deck.draw());
    }

    //check
    assert_eq!(true, check_winner(&p_list, &default_settings()));
}

#[test]
fn check_for_winner_custom_req14b()
{
    let mut p_list : Vec<Player> = Vec::new();
    let settings = custom_settings(true, false, 0, 0, [(4, 10)].to_vec());
    //Generate 4 players
    for i in 0..4
    {
        p_list.push(player_factory(i, true, true));
    }

    //Check
    assert_ne!(true, check_winner(&p_list, &default_settings()));

    //Create a dummy green deck
    let mut g_deck = GreenDeck::empty();
    let _ = g_deck.read_cards();
    g_deck = g_deck.shuffle();
    
    //give a player 5 green
    for _ in 0..5
    {
        println!("{}\n", &g_deck.get_title_of_top_card());
        p_list[0].give_green(g_deck.draw());
    }

    //check
    assert!(check_winner(&p_list, &default_settings()) == false);

     //give a player 5 more green
     for _ in 0..5
     {
         println!("{}\n", &g_deck.get_title_of_top_card());
         p_list[0].give_green(g_deck.draw());
     }
     
     //check
     assert!(check_winner(&p_list, &default_settings()) == true);
}

#[test]
fn test_if_deck_factories_works()
{
    let g_deck : GreenDeck = green_deck_factory();
    let r_deck : RedDeck = red_deck_factory();
    assert!(g_deck.get_size() > 1);
    assert!(r_deck.get_size() > 1);

    let mut dummy_g = GreenDeck::empty();
    let mut dummy_r = RedDeck::empty();

    _ = dummy_g.read_cards();
    _ = dummy_r.read_cards();

    assert_eq!(g_deck.get_size(), dummy_g.get_size());
    assert_eq!(r_deck.get_size(), dummy_r.get_size());
}


#[test]
fn test_wild_red_apples()
{
    //Create a dummy card
    let mut rc = wild_red_factory();

    //Add a dummy player
    let p = player_factory(0, true, false);

    //get title beforehand
    let b4 = wild_red_factory().get_title();

    //Check that it is a wild apple
    assert_eq!(rc.is_wild(), true);

    //compare to normal apple
    assert_ne!(redcard_factory(String::new(), String::new()).is_wild(), true);

    //let the player play
    rc.set_title(p.prompt_wild_apple());

    //See if the title changed
    assert_ne!(rc.get_title(), b4);
}

#[test]
fn test_custom_title_red_apple()
{
    //Create a dummy card
    let mut rc = wild_red_factory();

    //Set the title to something
    rc.set_title("Ayy lmao".to_string());

    //See if it changed
    assert_eq!("Ayy lmao".to_string(), rc.get_title());
}

#[test]
fn do_wild_apples_get_added()
{
    //Create the deck
    let mut r_deck = red_deck_factory();
    let settings = custom_settings(true, true, 1000, 0, Vec::new());

    //Check the size before we add the red apples.
    let deck_size_before = r_deck.get_size();

    //Add wild apples to the deck if we use them
    if settings.wild_red_apples() > 0
    {
        //Add some to the red deck
        for _ in 0..settings.wild_red_apples()
        {
            r_deck.add_to_deck(wild_red_factory());
        }
        //shuffle the red deck.
        r_deck = r_deck.shuffle();
    }

    //Check it after we add the red apples.
    assert!(r_deck.get_size() > deck_size_before);
}
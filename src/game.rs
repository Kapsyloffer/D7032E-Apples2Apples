use crate::player::*;
use crate::deck::*;
use crate::card::*;
use std::collections::HashMap;
use rand::Rng;

pub fn init_game()
{
    //TODO:: HOST?
    //Create all the decks
    let mut r_deck = RedDeck{cards: Vec::new()};
    let mut g_deck = GreenDeck{cards: Vec::new()};
    let mut d_deck = Discard{cards: Vec::new()};
    //Players
    let mut p_list : Vec<Player> = Vec::new();
    //add dummy players
    for i in 0..4
    {
        p_list.push(player_factory(i, true, false));
    }
    //setup
    game_setup(&mut r_deck, &mut g_deck);
    //gameplay
    gameplay(&mut r_deck, &mut g_deck, &mut d_deck, &mut p_list);
    //idk
}

fn game_setup(r_deck : &mut RedDeck,g_deck : &mut GreenDeck)
{

    //1. Read all of the green apples
    let _q = r_deck.read_cards();
    //2. Read all of the red apples 
    let _q = g_deck.read_cards();

    //3. Shuffle both of the decks 
    r_deck.shuffle();
    g_deck.shuffle();

    //Just to check the size of em.
    println!("{}", &r_deck.cards.len().to_string());
    println!("{}", &g_deck.cards.len().to_string());
    //Gameplay
    //5. Pick a judge at random.
    //TODO: player_list[rnd(1..size)] eller nåt
    //while true:
    //green card picked at random
    //Alla spelar kort, except the judge
    //Shuffle answers
    //judge picks
    //winner gets green cards
    //next(judge)
}


#[allow(dead_code)]
#[allow(unused_variables)]
fn gameplay(r_deck : &mut RedDeck, g_deck : &mut GreenDeck, d_deck : &mut Discard, p_list : &mut Vec<Player>)
{
    let mut judge : Player;
    let mut cur_green : GreenCard;
    //Include id so we can track the winner
    let mut red_cards : HashMap<i32, RedCard> = HashMap::new();
    #[allow(while_true)]
    while true
    {
        //pick judge
        judge = judge_pick(&p_list).clone();
        println!("{} is the Judge!", judge.get_id().to_string());
        //6. A green apple is drawn from the pile 
        cur_green = g_deck.cards.remove(0);
        //and shown to everyone
        print!("{}\n{}", cur_green.title, cur_green.desc);
        //7. All players except the judge plays a red Apple
        for p in p_list.iter_mut()
        {
            if &p.get_id() != &judge.get_id()
            {
                //play_cards return a redcard, the idea is that
                //the played redcard will go straight into the pile
                red_cards.insert(p.get_id(), p.play_card()); 
            }
        }
        //I guess we do a foreach player, and have them pick a card. Skipping the Judge.
        //8. Order is randomized before shown.
        //TODO: Somehow grab the hashmap, shuffle it, and show to the judge?
        //Maybe make a function like judge.pick_card(cardlist)
        //in case we play voting we do foreach p in p_list, p.vote(p_list)
        //9. All players must play a card before the results at 8 are shown.
        if &red_cards.len()-0 == &p_list.len()-1
        {
            todo!()
        }
        //10a. Judge picks card, winner gets the green apple.
        judge.pick(&mut red_cards);
        //10b. OR WE VOTE, however you cannot vote on your own
        //11. All red apples end up in the discard pile.
        for rc in red_cards.clone()
        {
            r_deck.add_to_deck(rc.1);
        }
        red_cards.clear(); //Ful lösning, men jag löser den I guess :P
        //Check if winner, else continue
        if check_winner(&p_list)
        {
            break;
        }
        //12. All players draw 7-n cards where n is their handsize
        //13. Next player in the list becomes judge.
    }
}

pub fn refill_hand(p : &mut Player, red_deck : &mut RedDeck) //TODO: Change to REDDECK
{
    while p.get_hand_size() < 7
    {
        p.add_to_hand(red_deck.draw());
    }
}

pub fn judge_pick(p_list : &Vec<Player>) -> &Player
{
    let selected_index = rand::thread_rng().gen_range(0..p_list.len());
    return &p_list[selected_index];
}

pub fn next_judge<'a>(p_list: &'a Vec<Player>, cur_judge: &'a Player) -> &'a Player
{
    let mut i = 0;

    //Get index of current judge
    for p in p_list
    {
        if cur_judge.get_id() == p.get_id()
        {
            break;
        }
        i = i+1;
    }
    let next_judge : &Player;

    //if id overflows, return to 0
    if i == p_list.len() - 1 
    {
        next_judge = &p_list[0];
    }
    else
    {
        next_judge = &p_list[i+1];
    }
    return next_judge;
}

pub fn check_winner(p_list : &Vec<Player>) -> bool
{
    let limit : u8;

    match p_list.len()
    {
        4=>limit = 8,
        5=>limit = 7,
        6=>limit = 6,
        7=>limit = 5,
        l if l >= 8 => limit = 4, //8+ players
        _=> panic!("Wtf u doing bro") // <4 players
    }
    for p in p_list
    {
        if p.get_green_amount() >= limit
        {
            return true;
        }
    }
    return false;
}
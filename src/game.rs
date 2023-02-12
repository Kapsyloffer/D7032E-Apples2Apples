use crate::player::*;
use crate::deck::*;
//use crate::card::*;
use rand::Rng;

pub fn init_game()
{
    //Create all the decks
    let mut r_deck = RedDeck{cards: Vec::new()};
    let mut g_deck = GreenDeck{cards: Vec::new()};
    let mut d_deck = Discard{cards: Vec::new()};
    //Players
    let mut p_list : Vec<Player> = Vec::new();
    game_setup(&mut r_deck, &mut g_deck);
    gameplay(&mut r_deck, &mut g_deck, &mut d_deck, &mut p_list);
    //plyaerlist
    //host
    //setup
    //gameplay
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
    let _judge : Player = judge_pick(&p_list);
    #[allow(while_true)]
    while true
    {
        //if judge is empty, pick judge
        //end of game, next jduge
        //6. A green apple is drawn from the pile and shown to everyone
        //7. All players except the judge plays a red Apple
        //8. Order is randomized before shown.
        //9. All players must play a card before the results at 8 are shown.
        //10. Judge picks card, winner gets the green apple.
        //11. All red apples end up in the discard pile.
        //12. All players draw 7-n cards where n is their handsize
        //13. Next player in the list becomes judge.
        if check_winner(&p_list)
        {
            break;
        }
    }
}

pub fn refill_hand(p : &mut Player, red_deck : &mut RedDeck) //TODO: Change to REDDECK
{
    while p.get_hand_size() < 7
    {
        p.add_to_hand(red_deck.draw());
    }
}

pub fn judge_pick(p_list : &Vec<Player>) -> Player
{
    let selected_index = rand::thread_rng().gen_range(0..p_list.len());
    return p_list[selected_index].clone();
}

pub fn next_judge(p_list : &Vec<Player>, cur_judge : &Player) -> Player
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
    let next_judge : Player;

    if i == p_list.len() - 1 
    {
        next_judge = p_list[0].clone();
    }
    else
    {
        next_judge = p_list[i+1].clone();
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
        l if l >= 8 => limit = 4, //8+
        _=> panic!("Wtf u doing bro")
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
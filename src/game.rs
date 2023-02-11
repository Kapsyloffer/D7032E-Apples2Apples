use crate::player::*;
use crate::deck::*;
//use crate::card::*;
use rand::Rng;

pub fn init_game()
{
    game_setup();
    gameplay();
    //plyaerlist
    //host
    //setup
    //gameplay
    //idk
}

fn game_setup()
{
    //Create all the decks
    let mut r_deck = RedDeck{cards: Vec::new()};
    let mut g_deck = GreenDeck{cards: Vec::new()};
    let mut _d_deck = Discard{cards: Vec::new()};

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
 
    //4. Deal 7 red apples to each player
    #[allow(unused_mut)]
    let mut p_list : Vec<Player> = Vec::new();

    //TODO: foreach player, add to playerlist, send playerlist into deck and do the deal.
    for p in p_list.clone()
    {
        //points wont work :(
        let dif = 7 - &p.get_hand_size();

        refill_hand(p, r_deck.clone());

        for _ in 1..dif
        {
            r_deck.cards.remove(0);
        }
    }

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
fn gameplay()
{
    //let _judge : Player = judge_pick(&p_list);
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

        /*Here’s how to tell when the game is over:
        • For 4 players, 8 green apples win.
        • For 5 players, 7 green apples win.
        • For 6 players, 6 green apples win.
        • For 7 players, 5 green apples win.
        • For 8+ players, 4 green apples win.*/
        break;
    }
}

pub fn refill_hand(mut p : Player, mut red_deck : RedDeck) -> Player //TODO: Change to REDDECK
{
    while p.get_hand_size() < 7
    {
        p.add_to_hand(red_deck.draw());
    }
    return p;
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
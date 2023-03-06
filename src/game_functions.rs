use crate::player::*;
use crate::cardpiles::*;
use crate::card::*;
use crate::settings::*;

use rand::Rng;
extern crate colorize;
use colorize::*;


//Have all players draw up to 7 from anywhere.
pub fn refill_hand(p : &mut Player, red_deck : &mut RedDeck, s : &Settings)
{
    while p.get_hand_size() < s.get_max_hand_size()
    {
        p.add_to_hand(red_deck.draw());
    }
}

//Pick one judge at random.
pub fn judge_pick(p_list : &Vec<Player>) -> &Player
{
    let selected_index = Some(rand::thread_rng().gen_range(0..p_list.len()));
    return &p_list[selected_index.unwrap()];
}

//Picks out the next judge in the list. Resets to 0 if needed.
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

//Check winner requirement at the end of each game.
pub fn check_winner(p_list : &Vec<Player>, settings: &Settings) -> bool
{
    //Score limit to win
    let mut limit : Option<i32> = None;
    //Size of players
    let playersize: i32 = p_list.len() as i32;

    //s is a Vec<(i32, i32)>. Where s.0 is the playercount, and s.1 is the score needed to win.
    //hence this function.
    for s in settings.get_winreq()
    {
        if playersize == s.0 && playersize < 8
        {
            limit = Some(s.1);
            break;
        }
        else if playersize >= 8 && s.0 >= 8
        {
            limit = Some(s.1);
            break;
        }
    }
    if limit.is_none()
    {
        panic!("Limit broke"); //This breaks if settings break.
    }
    //Compare if there are any winners.
    for p in p_list
    {
        if p.get_green_amount() >= limit.unwrap() as u8
        {
            //Print this if we get a winner.
            println!("\n====== {}{} ======", p.get_id().to_string().green().bold(), " IS THE WINNER!!".bold().green());
            //If we get a winner, return true and the game is over.
            return true;
        }
    }
    //Else we return false as there are no winners.
    return false;
}

//Add all of the played cards to discard, returns new discard for testing.
pub fn send_to_discard(rc: Vec<(i32, RedCard)>, d : &mut Discard) -> Vec<(i32, RedCard)>
{
    for (_, c) in rc
    {
        d.add_to_discard(c);
    }
    //Return an empty vector.
    return Vec::new(); //Ful lösning but eh
}

//Disallows judge from playing apple.
pub fn can_play_apple(p: &Player, j : &Player) -> bool
{
    return p.get_id() != j.get_id();
}

//Give the winner a green card.
pub fn reward_winner(win : &mut Player, green : GreenCard)
{
    win.give_green(green);
}

//As per requirement 8 we have to shuffle the cards before showing.
pub fn shuffle_before_showing(cards: &Vec<(i32, RedCard)>) -> Vec<(i32, RedCard)> {
    //Fisher Yates shuffle algorithm
    let mut shuffled_deck = cards.clone();
    let size = shuffled_deck.len();

    for i in 0..size {
        //Select random element
        let j = rand::thread_rng().gen_range(i..size);
        //Swap element at position i with element at position j
        shuffled_deck.swap(i, j);
    }

    // Return the shuffled deck
    return shuffled_deck;
}

//Draw a new green card. Req 6
pub fn new_green(g_deck : &mut GreenDeck) -> GreenCard
{
    return g_deck.draw();
}

//Count the votes and return the winner ID.
pub fn count_votes(p_list : &Vec<Player>, vote_counter : &mut Vec<i32>) -> i32
{
    // Count votes
    let mut max_votes = 0;
    let mut max_player_ids: Vec<i32> = Vec::new();
    for p in p_list.iter() 
    {
        let vote_count = vote_counter.iter().filter(|&n| *n == p.get_id()).count();
        if vote_count > max_votes 
        {
            max_votes = vote_count;
            max_player_ids = vec![p.get_id()];
        } 
        else if vote_count == max_votes 
        {
            max_player_ids.push(p.get_id());
        }
    }

    // If there's a tie, randomly choose a player to eliminate
    if max_player_ids.len() > 1 
    {
        let random_index = rand::thread_rng().gen_range(0 .. max_player_ids.len());
        return max_player_ids[random_index];
        // eliminate the player with the specified ID
    }
    else 
    {
        return max_player_ids[0];
    }
}

pub fn print_standings(p_list : &Vec<Player>)
{
    println!("\n====== {} ======", "STANDINGS".yellow());
        for p in p_list
        {
            println!("Player {} has: {} GREENS", p.get_id().to_string().yellow(), p.get_green_amount().to_string().green());
        }
        println!("\n");
}
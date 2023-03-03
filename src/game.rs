use crate::player::*;
use crate::cardpiles::*;
use crate::card::*;
use crate::settings::*;

use rand::Rng;
extern crate colorize;
use colorize::*;



//TODO: Future modifications
/*
- A phase before A that lets people discard cards from their hands. --DONE
- Gamemode, either Judge or Vote. --DONE
- Wild red apples. --TODO: Rewrite redDeck and make prompt for wild red apple
 */

//Some simple setup.
pub fn init_game(settings : Settings)
{
    //TODO:: HOST?

    //Create all the decks
    let mut r_deck = RedDeck{cards: Vec::new()};
    let mut g_deck = GreenDeck{cards: Vec::new()};
    let mut d_deck = Discard{cards: Vec::new()};
    
    //Add Players
    let mut p_list : Vec<Player> = Vec::new();

    //Add default settings
    //let settings : Settings = custom_settings(false, true, 0);

    //THE player, somehow it fixes the unshuffled deck bug
    p_list.push(player_factory(0, false, true));
    //add dummy players
    for i in 1..(settings.get_bots() as i32) +1
    {
        p_list.push(player_factory(i, true, false)); //TODO: Real players
    }

    //gameplay
    gameplay(&mut r_deck, &mut g_deck, &mut d_deck, &mut p_list, settings);

    //And that's all she wrote.
}

//Main gameplayloop happens here.
fn gameplay(r_deck : &mut RedDeck, g_deck : &mut GreenDeck, d_deck : &mut Discard, p_list : &mut Vec<Player>, settings: Settings)
{
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    //Req 1. Read all of the green apples
    //Req 2. Read all of the red apples 
    _ = g_deck.read_cards();
    _ = r_deck.read_cards();

    if settings.wild_red_apples() > 0
    {
        //Add some to the red deck
        todo!()
    }

    //Req 3. Shuffle both of the decks
    *r_deck = r_deck.shuffle();
    *g_deck = g_deck.shuffle();

    //This will always be the shown green card.
    let mut cur_green : GreenCard;

    //Include id so we can track the winner
    let mut red_cards : Vec<(i32, RedCard)> = Vec::new();

    //Req 4. deal 7 cards to each player
    for p in p_list.iter_mut()
    {
        refill_hand(p, r_deck);
    }

    //Req 5. pick judge
    let mut judge : Player; 

    if settings.use_judge()
    {
        judge = judge_pick(&p_list).clone();
    }
    else 
    {
        //create a dummy judge if we use votes.
        judge = player_factory(9999, true, true); 
    }

    loop
    {
        //"Clear" the screen
        //print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        //before phase A, prompt discard
        if settings.use_discard()
        {
            for p in p_list.iter_mut()
            {
                p.prompt_discard(d_deck);
                refill_hand(p, r_deck);
            }
        }

        //If we use judge, announce, else nah.
        if settings.use_judge()
        {
            //Announce the judge
            println!("{} is the Judge!\n", judge.get_id().to_string().yellow());
        }

        //Req 6. A green apple is drawn from the pile 
        cur_green = new_green(g_deck);

        //and shown to everyone
        println!(" {}\n{}\n", &cur_green.get_title().green().bold(), &cur_green.get_desc().green()); 
        
        //Req 7. All players except the judge plays a red Apple
        //Req9. All players must play a card before the results at 8 are shown.
        for p in p_list.iter_mut()
        {
            if can_play_apple(&p, &judge)
            {
                //play_cards return a redcard, the idea is that
                //the played redcard will go straight into the pile
                red_cards.push((p.get_id(), p.play_card())); 
            }
        }

        //Req 8. Order is randomized before shown.
        red_cards = shuffle_before_showing(&mut red_cards);

        let winner : i32;

        if settings.use_judge()
        {
            //Req 10a. Judge picks card, winner gets the green apple. Also shuffle order before showing.
            winner = judge.pick(&mut red_cards);
        }
        else
        {
            //Req 10b. OR WE VOTE, however you cannot vote on your own (which is handled in player.rs)'
            
            let mut vote_counter : Vec<i32> = Vec::new();

            //Have everyone vote
            for p in p_list.iter_mut()
            {
                vote_counter.push(p.vote(&mut red_cards, &cur_green));
            }
            winner = count_votes(&p_list, &mut vote_counter);
        }

        //Announce winner
        print!("The winner is {}!", &winner.to_string().yellow());

        //Give the winner the current green card. One potential problem here is that
        //if one id gets disconnected it breaks.
        reward_winner(&mut p_list[winner as usize], cur_green);
        
        //Req 11. All red apples end up in the discard pile.
        red_cards = send_to_discard(red_cards, d_deck);

        //Req 14? check if winner, else continue
        if check_winner(&p_list, &settings)
        {
            println!("\n====== {}{} ======", &winner.to_string().green().bold(), " IS THE WINNER!!".bold().green());
            break;
        }

        //Show standings. Not a requirement of anything, just fun.
        println!("\n====== {} ======", "STANDINGS".yellow());
        for p in p_list.iter_mut()
        {
            println!("Player {} has: {} GREENS", p.get_id().to_string().yellow(), p.get_green_amount().to_string().green());
        }
        println!("\n");


        //Req 12. All players draw 7-n cards where n is their handsize
        for p in p_list.iter_mut()
        {
            refill_hand(p, r_deck);
        }

        if settings.use_judge()
        {
            //Req 13. Next player in the list becomes judge.
            judge = next_judge(p_list, &judge).clone(); //TODO: FIX  
        }
    }
}

//Have all players draw up to 7 from anywhere.
pub fn refill_hand(p : &mut Player, red_deck : &mut RedDeck)
{
    while p.get_hand_size() < 7
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
    let mut limit : Option<i32> = None;
    let playersize: i32 = p_list.len() as i32;
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
        panic!("Limit broke");
    }
    for p in p_list
    {
        if p.get_green_amount() >= limit.unwrap() as u8
        {
            return true;
        }
    }
    return false;
}

//Add all of the played cards to discard, returns new discard for testing.
pub fn send_to_discard(rc: Vec<(i32, RedCard)>, d : &mut Discard) -> Vec<(i32, RedCard)>
{
    for (_, c) in rc
    {
        d.add_to_discard(c);
    }
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
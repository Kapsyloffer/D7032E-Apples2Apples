use crate::player::*;
use crate::cardpiles::*;
use crate::card::*;
use crate::settings::*;
use crate::game_functions::*;

extern crate colorize;
use colorize::*;

//Some simple setup.
pub fn init_game(settings : Settings)
{
    //TODO:: HOST?

    //Create all the decks
    let mut r_deck = red_deck_factory();
    let mut g_deck = green_deck_factory();
    let mut d_deck = discard_factory();

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
    
    //Add Players
    let mut p_list : Vec<Player> = Vec::new();

    //THE player, somehow it fixes the unshuffled deck bug
    p_list.push(player_factory(0, false, true));

    //add dummy players
    for i in 1..(settings.get_bots() as i32) +1
    {
        p_list.push(player_factory(i, true, false)); //TODO: Real players
    }

    //Req 4. deal 7 cards to each player
    for p in p_list.iter_mut()
    {
        refill_hand(p, &mut r_deck, &settings);
    }

    //gameplay
    gameplay(&mut r_deck, &mut g_deck, &mut d_deck, &mut p_list, settings);
}

//Main gameplayloop happens here.
fn gameplay(r_deck : &mut RedDeck, g_deck : &mut GreenDeck, d_deck : &mut Discard, p_list : &mut Vec<Player>, settings: Settings)
{
    //Clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    //The shown green card. Changes
    let mut cur_green : GreenCard;

    //Pile of played red cards, 0:id, 1:The card; so we can track the winner
    let mut red_cards : Vec<(i32, RedCard)> = Vec::new();

    //Req 5. pick judge at random (Will not be used if we use votes.)
    let mut judge : Player = judge_pick(&p_list).clone(); 

    //Gameplay loop, this keeps running until we're done.
    loop
    {
        //before phase A, prompt discard if we use it.
        if settings.use_discard()
        {
            for p in p_list.iter_mut()
            {
                //Discard
                p.prompt_discard(d_deck);
                //Refill hand
                refill_hand(p, r_deck, &settings);
            }
        }

        //If we use judge, announce him, else nah.
        if settings.use_judge()
        {
            //Announce the judge
            println!("{} is the Judge!\n", judge.get_id().to_string().yellow());
        }

        //Req 6. A green apple is drawn from the pile...
        cur_green = new_green(g_deck);

        //...and shown to everyone
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

        //Holds the ID of the winner of Vote or Judge pick
        let winner : i32;

        if settings.use_judge()
        {
            //Req 10a. Judge picks card, winner gets the green apple. Also shuffle order before showing.
            winner = judge.pick(&mut red_cards);
        }
        else
        {
            //Req 10b. OR WE VOTE, however you cannot vote on your own card (which is handled in player.rs)'
            
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
            break;
        }

        //Show standings. Not a requirement of anything, just fun.
        print_standings(&p_list);

        //Req 12. All players draw 7-n cards where n is their handsize
        for p in p_list.iter_mut()
        {
            refill_hand(p, r_deck, &settings);
        }

        //Req 13. Next player in the list becomes judge.
        if settings.use_judge()
        {
            judge = next_judge(p_list, &judge).clone();   
        }
    }
}
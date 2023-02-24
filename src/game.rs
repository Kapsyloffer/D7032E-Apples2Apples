use crate::player::*;
use crate::cardpiles::*;
use crate::card::*;
use rand::Rng;

pub fn init_game()
{
    //TODO:: HOST?

    //Create all the decks
    let mut r_deck = RedDeck{cards: Vec::new()};
    let mut g_deck = GreenDeck{cards: Vec::new()};
    let mut d_deck = Discard{cards: Vec::new()};
    
    //Add Players
    let mut p_list : Vec<Player> = Vec::new();

    //add dummy players
    for i in 0..5
    {
        p_list.push(player_factory(i, true, false));
    }
    p_list.push(player_factory(5, false, true));

    //gameplay
    gameplay(&mut r_deck, &mut g_deck, &mut d_deck, &mut p_list);

    //And that's all she wrote.
}


#[allow(dead_code)]
#[allow(unused_variables)]
fn gameplay(r_deck : &mut RedDeck, g_deck : &mut GreenDeck, d_deck : &mut Discard, p_list : &mut Vec<Player>)
{
    
    //Req 1. Read all of the green apples
    //Req 2. Read all of the red apples 
    _ = g_deck.read_cards();
    _ = r_deck.read_cards();

    for i in 0..9
    {
        println!("{}", r_deck.get_top_card_title(i));
    }
    println!("");
    //Req 3. Shuffle both of the decks (Doesn't work for some reason)
    *r_deck = r_deck.shuffle();
    *g_deck = g_deck.shuffle();

    for i in 0..9
    {
        println!("{}", r_deck.get_top_card_title(i));
    }

    //This will always be the shown green card.
    let mut cur_green : GreenCard;

    //Include id so we can track the winner
    let mut red_cards : Vec<(i32, RedCard)> = Vec::new();

    //Req 4. deal 7 cards to each player
    for p in p_list.iter_mut()
    {
        refill_hand(p, r_deck);
    }
    for p in p_list.iter_mut()
    {
        for x in &p.hand
        {
            println!("{}: {}", p.get_id().to_string(), x.title);
        }
        println!("\n");
    }

    //Req 5. pick judge
    let mut judge : Player = judge_pick(&p_list).clone();
    loop
    {
        //"Clear" the screen
        //print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        //Announce judge
        println!("{} is the Judge!\n", judge.get_id().to_string());

        //Req 6. A green apple is drawn from the pile 
        cur_green = g_deck.cards.remove(0);
        //and shown to everyone
        println!("{}\n{}", &cur_green.get_title(), &cur_green.get_desc()); //After this point, "cannot sample empty range"
        
        //Req 7. All players except the judge plays a red Apple
        for p in p_list.iter_mut()
        {
            if can_play_apple(&p, &judge)
            {
                //play_cards return a redcard, the idea is that
                //the played redcard will go straight into the pile
                red_cards.push((p.get_id(), p.play_card())); //<-- Troublemaker
            }
        }

        //Req 8. Order is randomized before shown.
        //TODO: Somehow grab the vec, shuffle it, and show to the judge?

        //Maybe make a function like judge.pick_card(cardlist)
        //in case we play voting we do foreach p in p_list, p.vote(p_list)

        //Req9. All players must play a card before the results at 8 are shown.
        /*if &red_cards.len()-0 == &p_list.len()-1 //if we use judge
        {
            todo!()
        }*/

        //Req 10a. Judge picks card, winner gets the green apple.
        let winner : i32 = judge.pick(&mut red_cards);
        //println!("\n\nTHE WINNER IS {} who played:\n{}", &winner.to_string(), &red_cards.get(&winner).unwrap().get_title());
        reward_winner(&mut p_list[winner as usize], cur_green);
        
        //Req 10b. OR WE VOTE, however you cannot vote on your own
        
        //Req 11. All red apples end up in the discard pile.
        red_cards = send_to_discard(red_cards, d_deck);

        //Req 14? check if winner, else continue
        if check_winner(&p_list)
        {
            println!("{} IS THE WINNER!!", &winner.to_string());
            break;
        }

        //Show standings. Not a requirement of anything, just fun.
        println!("\nSTANDINGS:\n");
        for p in p_list.iter_mut()
        {
            println!("Player {} has: {} GREENS", p.get_id().to_string(), p.get_green_amount().to_string());
        }
        println!("\n");


        //Req 12. All players draw 7-n cards where n is their handsize
        for p in p_list.iter_mut()
        {
            refill_hand(p, r_deck);
        }

        //Req 13. Next player in the list becomes judge.
        judge = next_judge(p_list, &judge).clone(); //TODO: FIX
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
    let selected_index = Some(rand::thread_rng().gen_range(0..p_list.len()));
    return &p_list[selected_index.unwrap()];
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

pub fn send_to_discard(rc: Vec<(i32, RedCard)>, d : &mut Discard) -> Vec<(i32, RedCard)>
{
    for (_, c) in rc
    {
        d.add_to_discard(c);
    }
    return Vec::new(); //Ful lösning but eh
}

pub fn can_play_apple(p: &Player, j : &Player) -> bool
{
    return p.get_id() != j.get_id();
}

pub fn reward_winner(win : &mut Player, green : GreenCard)
{
    win.give_green(green);
}

pub fn shuffle_before_showing(cards: &mut Vec<(i32, RedCard)>) -> Vec<(i32, RedCard)>
{  
    //Fisher Yates shuffle algorithm
    let mut deck : Vec<(i32, RedCard)> = cards.clone();
    let size : u8 = deck.len() as u8;

    for i in 0..size
    {   
        //Select last element
        let j : (i32, RedCard) = deck.pop().unwrap();
        //rnd [0 -> size-i]
        let rnd : u8 = rand::thread_rng().gen_range(0..(size-i));
        //Switch element[size] with element[size-i]
        let k : (i32, RedCard) = deck[usize::from(rnd)].clone();
        deck[usize::from(rnd)] = j;
        deck.push(k);
    }
    //Set the current deck to the shuffled deck.
    return deck;
}
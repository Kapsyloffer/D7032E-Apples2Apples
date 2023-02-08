#[allow(unused_imports)]
use crate::game::*;
#[allow(unused_imports)]
use crate::player::*;
#[allow(unused_imports)]
use crate::card::*;
#[allow(unused_imports)]
use crate::deck::*;    

#[cfg(test)]
//SETUP
#[test]
fn test_read_all_green_apples() //Req 1
{
    let mut _gc : GreenDeck = GreenDeck
    {
        cards : Vec::new()
    };
    let b4 : i32 = _gc.cards.len().try_into().unwrap();
    _gc.read_cards();
    let after : i32 = _gc.cards.len().try_into().unwrap();
    assert_ne!(b4, after);
}
#[test]
fn test_read_all_red_apples()  //Req 2
{
    let mut _rc : RedDeck = RedDeck
    {
        cards : Vec::new()
    };
    let b4 : i32 = _rc.cards.len().try_into().unwrap();
    _rc.read_cards();
    let after : i32 = _rc.cards.len().try_into().unwrap();
    assert_ne!(b4, after);
}
#[test]
fn test_shuffle_both_decks()  //Req 3
{
    let mut rc : RedDeck = RedDeck
    {
        cards : Vec::new()
    };
    rc.read_cards();
    let b4shuffle : Vec<RedCard> = rc.cards.clone();
    rc.shuffle();
    print!("{}, {}", rc.cards[0].title, b4shuffle[0].title);
    assert_ne!(rc.cards[0].title, b4shuffle[0].title);
}

#[test]
fn test_deal7_red_apples_to_each_player()  //Req 4
{
    assert_eq!(1, 0);
}

#[test]
fn test_pick_a_judge_at_random()  //Req 5
{
    assert_eq!(1, 0);
}

//GAMEPLAY

#[test]
fn test_green_apple_drawn_and_shown_to_everyone()  //Req 6
{
    assert_eq!(1, 0);
}

#[test]
fn test_judge_do_not_play_red_apple()  //Req 7
{
    assert_eq!(1, 0);
}

#[test]
fn test_order_of_cards_random_before_shown()  //Req 8
{
    assert_eq!(1, 0);
}

#[test]
fn test_all_players_must_play_before_result_is_shown()  //Req 9
{
    assert_eq!(1, 0);
}

#[test]
fn test_judge_picks_card_winner_gets_green_apple()  //Req 10
{
    assert_eq!(1, 0);
}

#[test]
fn test_all_red_apples_go_to_discard()  //Req 11
{
    assert_eq!(1, 0);
}

#[test]
fn test_all_players_draw_up_to_seven()  //Req 12
{
    assert_eq!(1, 0);
}

#[test]
fn test_next_player_in_list_becomes_judge()  //Req 13
{
    assert_eq!(1, 0);
}

#[test]
fn test_4p_check_for_winner()  //Req 14
{
    assert_eq!(1, 0);
}

#[test]
fn test_5p_check_for_winner() 
{
    assert_eq!(1, 0);
}

#[test]
fn test_6p_check_for_winner() 
{
    assert_eq!(1, 0);
}

#[test]
fn test_7p_check_for_winner() 
{
    assert_eq!(1, 0);
}

#[test]
fn test_8plus_check_for_winner() 
{
    assert_eq!(1, 0);
}
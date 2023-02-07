use crate::game::*;
use player::*;
use card::*;
#[cfg(test)]
#[test]
fn add2test() 
{
    let b = 10;
    assert_eq!(b.clone() + 2, add2(b));
}

#[test]
fn HandTest() 
{
    let card = Card
    {
        content = "butts";
    }
    let player = Player
    {
        player_id : 0,
        is_bot : false,
        online : true,
        hand : Vec::<GreenCard>::new()
        green_apples : Vec<GreenCard>,
    }
}
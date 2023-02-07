use std::clone;

use crate::card::GreenCard;
#[allow(dead_code)]
pub struct Player
{
    player_id : i32,
    is_bot : bool,
    online : bool,
    //connection : socket,
    //inFromClient : BufferedReader,
    //outToClient : DataOutputStream,
    hand : Vec<GreenCard>,
    green_apples : Vec<GreenCard>,
}

impl Player
{
    #[allow(dead_code)]
    fn do_stuff(&self)
    {
        todo!();
    }

    fn get_hand(&self) -> Vec<GreenCard>
    {
        let mut e : Vec<GreenCard> = self.hand.clone();
        if e.is_empty()
        {
            e = Vec::<GreenCard>::new();
        }
        e
    }
}
use crate::card::*;
#[allow(dead_code)]
pub struct Player
{
    player_id : i32,
    is_bot : bool,
    online : bool,
    //connection : socket,
    //inFromClient : BufferedReader,
    //outToClient : DataOutputStream,
    hand : Vec<RedCard>,
    green_apples : Vec<GreenCard>,
}

impl Player
{
    #[allow(dead_code)]
    fn do_stuff(&self)
    {
        todo!();
    }
}
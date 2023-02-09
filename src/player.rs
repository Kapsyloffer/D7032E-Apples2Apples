use crate::card::*;
#[allow(dead_code)]
#[derive(Clone)]
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

pub trait PlayerActions
{
    fn play_card (&mut self) -> RedCard;
    fn get_green_amount (&self) -> u8;
    fn add_to_hand(&mut self, rc : RedCard);
    fn get_hand_size(&self) -> u8;
}

impl Player
{
    #[allow(dead_code)]
    fn do_stuff(&self)
    {
        todo!();
    }
}

impl PlayerActions for Player
{
    fn add_to_hand(&mut self, rc : RedCard) 
    {
        self.hand.push(rc);
    }

    fn get_green_amount (&self) -> u8 
    {
        let greens : u8 = self.green_apples.len() as u8;
        greens
    }

    fn get_hand_size(&self) -> u8 
    {
        let handsize : u8 = self.hand.len() as u8;
        handsize
    }

    fn play_card (&mut self) -> RedCard 
    {
        todo!()
    }
}

pub fn player_factory (id : i32, bot : bool, o: bool) -> Player
{
    let p : Player = Player
    {
        player_id : id,
        is_bot : bot,
        online : o,
        //connection : socket,
        //inFromClient : BufferedReader,
        //outToClient : DataOutputStream,
        hand : Vec::new(),
        green_apples : Vec::new(),
    };
    p
}
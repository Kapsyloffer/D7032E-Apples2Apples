
#[allow(dead_code)]
pub struct Player
{
    player_id : i32,
    is_bot : bool,
    online : bool,
    //connection : socket,
    //inFromClient : BufferedReader,
    //outToClient : DataOutputStream,
    hand : Vec<String>,
    green_apples : Vec<String>,
}

impl Player
{
    #[allow(dead_code)]
    fn do_stuff()
    {
        todo!();
    }
}
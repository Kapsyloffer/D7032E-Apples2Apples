#![allow(dead_code)]
#![allow(unused)]
use std::net::*;
pub struct Server
{
    //TODO!
    listener: TcpListener,
    port: u16,
}
pub struct Client
{
    //TODO!
    pub stream: TcpStream,
    pub ip : Ipv4Addr,
    pub port : u16,
}

impl Server
{
    fn start()
    {
        todo!()
    }
}

impl Client
{
    fn connect(&mut self)
    {
        match TcpStream::connect(format!("{}:{}", self.ip, self.port)) 
        {
            Ok(stream) => 
            {
                println!("Connected to {}:{}", self.ip, self.port);
            }
            Err(e) => 
            {
                println!("Error connecting to {}:{}", self.ip, self.port);
                println!("Error message: {}", e);
            }
        }
    }

    fn send_to_everyone(&mut self, _msg : String)
    {
        //I guess vi tar och gör så att servern bouncar alla messages.
        //Kanske ett command som "chat [blablabla] skickar till alla och resten är input?"
        todo!()
    }
}

pub fn client_factory(ip: Ipv4Addr, port: u16) -> Client
{
    return Client {stream: TcpStream::connect(format!("{}:{}", ip, port)).unwrap(), ip: ip, port: port};
}

/* 

Flow of logic:

Server:
Handle players, and gameplay, hella server sided.

Client:
Get inputs when needed and tell the client about their
player's hands and scores.

        Client ska kunna: Connecta till server, och skicka nummer, om connectionen bryts, Hen everyone has played send to all clients 
        berätta för servern att player är gone och gör till bot.


each player is assigned to a client, unless they're a bot, then they're handled by the server.

Upon gameplay hands are dealt to the players in client which are handled by server.

When asking to play a card, show all cards to the player (client) that own them

Ask for input, parse it, play

then send all cards to the judge and await input,

judge input gets parsed

WSen to all clients "Yo this dude won"
*/
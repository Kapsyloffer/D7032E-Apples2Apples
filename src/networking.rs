#![allow(dead_code)]
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
    pub stream: Option<TcpStream>,
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
                self.stream = Some(stream);
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
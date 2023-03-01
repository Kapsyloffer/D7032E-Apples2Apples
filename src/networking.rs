#![allow(dead_code)]
#![allow(unused)]
use std::net::*;
use std::io::*;
use std::net::*;
use std::thread;


const PORT: u16 = 2048;
pub struct Server 
{
    players: Vec<TcpStream>,
    listener: TcpListener,
}
pub struct Client 
{
    pub (self) stream: TcpStream,
    pub (self) name: String,
}

impl Server
{
    pub fn new() -> Result<Self> 
    {
        // Bind the listener to the specified port
        let listener = TcpListener::bind("127.0.0.1:8080")?;
        println!("Server listening on port 8080");

        Ok(Server { players: Vec::new(), listener })
    }
    
    pub fn run(&mut self) -> Result<()> 
    {
        loop 
        {
            // Accept incoming connections
            let (stream, _) = self.listener.accept()?;
            println!("New connection");

            // Add the new player to the list of players
            self.players.push(stream.try_clone()?);

            // Send a welcome message to the new player
            let mut writer = BufWriter::new(stream);
            writer.write(b"Welcome to the game!\n")?;
            writer.flush()?;
        }
    }

    pub fn start_game(&mut self) -> Result<()> 
    {
        // Send a message to all players to start the game
        let mut writer = BufWriter::new(&self.players[0]);
        writer.write(b"Starting game\n")?;
        writer.flush()?;
    
        // Read a number from each player and send a reply
        for i in 0..self.players.len() 
        {
            let mut reader = BufReader::new(&self.players[i]);
            let mut buf = String::new();
            reader.read_line(&mut buf)?;
            let num = buf.trim().parse::<i32>().map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
            let reply = if num % 2 == 0 
            {
                "even"
            } 
            else 
            {
                "odd"
            };
            let mut writer = BufWriter::new(&self.players[i]);
            writer.write(reply.as_bytes())?;
            writer.flush()?;
        }
    
        Ok(())
    }

    /*
    fn run(&self) 
    {
        // Create a TCP listener on the specified port
        let listener = TcpListener::bind(("127.0.0.1", PORT)).unwrap();
        println!("Server listening on port {}", PORT);
    
        // Define a vector to hold all connected clients
        let mut clients = Vec::new();
    
        // Loop forever, accepting client connections and adding them to the clients vector
        for stream in listener.incoming() 
        {
            let stream = stream.unwrap();
            let client = Self::handle_client(stream.try_clone().unwrap());
            clients.push(client);
    
            // If there are at least two clients connected, start the game
            if clients.len() >= 2 
            {
                // Send a message to all clients in the lobby
                for client in &mut clients {
                    let msg = format!("{} joined the lobby", client.name);
                    client.stream.write(msg.as_bytes()).unwrap();
                }
    
                // Wait for the host to start the game
                let mut host_stream = clients[0].stream.try_clone().unwrap();
                loop 
                {
                    let mut buf = [0; 128];
                    host_stream.read(&mut buf).unwrap();
                    let cmd = String::from_utf8_lossy(&buf).trim_end().to_owned();
                    if cmd == "start" 
                    {
                        break;
                    }
                }
    
                // Send a message to all clients that the game is starting
                for client in &mut clients 
                {
                    let msg = "Starting game".to_owned();
                    client.stream.write(msg.as_bytes()).unwrap();
                }
    
                // Prompt each client for a number and act accordingly
                let mut numbers = Vec::new();

                for client in &mut clients 
                {
                    let msg = format!("{}: Enter a number", client.name);
                    client.stream.write(msg.as_bytes()).unwrap();
                    let mut buf = [0; 128];
                    client.stream.read(&mut buf).unwrap();
                    let num = String::from_utf8_lossy(&buf).trim_end().to_owned().parse().unwrap();
                    numbers.push(num);
                }

                let sum: i32 = numbers.iter().sum();
                let msg = format!("The sum is {}", sum);

                for client in &mut clients 
                {
                    client.stream.write(msg.as_bytes()).unwrap();
                }
            }
        }
    }

    // Define a function to handle client connections
    pub fn handle_client(mut stream: TcpStream) -> Client 
    {
        // Read the client's name from the stream
        let mut name_buf = [0; 128];
        stream.read(&mut name_buf).unwrap();
        let name = String::from_utf8_lossy(&name_buf).trim_end().to_owned();

        // Return a new Client struct with the stream and name
        Client { stream, name }
    } */
}

impl Client
{
    pub fn connect() -> Result<Self>
    {
        // Connect to the server on the specified port
        let mut stream = TcpStream::connect("127.0.0.1:8080")?;
        println!("Connected to server");

        // Prompt the user for their name and send it to the server
        print!("Enter your name: ");
        stdout().flush()?;
        let mut name = String::new();
        stdin().read_line(&mut name)?;
        stream.write(name.as_bytes())?;

        Ok(Client{name, stream})
    }

    pub fn play(&mut self) -> Result<()>
    {
        // Read messages from the server and print them to the console
        let mut buf = [0; 128];
        loop 
        {
            let bytes_read = self.stream.read(&mut buf)?;
            if bytes_read == 0 {
                // If the server closes the connection, exit the loop
                break;
            }
            let msg = String::from_utf8_lossy(&buf[..bytes_read]);
            println!("{}", msg);

            //TODO MODIFY
            if msg.contains("Starting game") 
            {
                // If the server starts the game, prompt the user for a number and send it to the server
                print!("Enter a number: ");
                stdout().flush()?;
                let mut num_str = String::new();
                stdin().read_line(&mut num_str)?;
                let num = num_str.trim().parse::<i32>().unwrap();
                self.stream.write(num.to_string().as_bytes())?;
            }
        }
    Ok(())
    }

    fn send_to_everyone(&mut self, _msg : String)
    {
        //I guess vi tar och gör så att servern bouncar alla messages.
        //Kanske ett command som "chat [blablabla] skickar till alla och resten är input?"
        todo!()
    }
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
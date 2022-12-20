import java.util.*; 
import java.nio.charset.StandardCharsets; 
import java.nio.file.*; 
import java.io.*; 
import java.net.*;
import java.util.concurrent.*;


class Player {
	public int playerID;
	public boolean isBot;
	public boolean online;
	public Socket connection;
	public BufferedReader inFromClient;
	public DataOutputStream outToClient;
	public ArrayList<String> hand;
	public ArrayList<String> greenApples = new ArrayList<String>();
	public Player(int playerID, ArrayList<String> hand, boolean isBot) {
		this.playerID = playerID; this.hand = hand; this.isBot = isBot; this.online = false;
	}
	public Player(int playerID, boolean isBot, Socket connection, BufferedReader inFromClient, DataOutputStream outToClient) {
		this.playerID = playerID; this.isBot = isBot; this.online = true;
		this.connection = connection; this.inFromClient = inFromClient; this.outToClient = outToClient;
	}

	public void play() {
		if(isBot) {
			/** BUG - FIX LATER
			 * For some reason I must sleep a random amount of time 
			 * or the playedApple ArrayList won't get all bot answers
 			 * (The teacher knows what the bug is, but thought this was fun to do :-)   )
			 **/
			Random rnd = ThreadLocalRandom.current();
			try{Thread.sleep(rnd.nextInt(500));}catch(Exception e){}
			// continue with non-buggy code

			Apples2Apples.playedApple.add(new PlayedApple(playerID, hand.get(0)));
			hand.remove(0);
		} else if(online){
			try {
				String aPlayedApple = inFromClient.readLine();
				Apples2Apples.playedApple.add(new PlayedApple(playerID, aPlayedApple));					
			} catch (Exception e) {}
		} else { //Server player, no separate thread needed since the server player always acts last
			System.out.println("Choose a red apple to play");
			for(int i=0; i<hand.size(); i++) {
				System.out.println("["+i+"]   " + hand.get(i));
			}
			System.out.println("");

			int choice = 0;
			try {
				BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
				String input=br.readLine();
				choice = Integer.parseInt(input);
			} catch (NumberFormatException e) {
				System.out.println("That is not a valid option");
				play();
			} catch (Exception e) {}
			Apples2Apples.playedApple.add(new PlayedApple(playerID, hand.get(choice)));
			hand.remove(choice);
			System.out.println("Waiting for other players\n");	
		}
	}

	public PlayedApple judge() {
		if(isBot){
			return Apples2Apples.playedApple.get(0);
		} else if(online){
			int playedAppleIndex = 0;
			try {
				playedAppleIndex = Integer.parseInt(inFromClient.readLine());	
			} catch(Exception e) {}
			return Apples2Apples.playedApple.get(playedAppleIndex);
		}  else {
			System.out.println("Choose which red apple wins\n");
			int choice = 0;
			try {
				BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
				String input=br.readLine();
				choice = Integer.parseInt(input);
			} catch (NumberFormatException e) {
				System.out.println("That is not a valid option");
				judge();
			} catch (Exception e) {}
			return Apples2Apples.playedApple.get(choice);			
		}
	}

	public void addCard(String redApple) {
		if(isBot || !online) {
			hand.add(redApple);
		} else {
			try {
				outToClient.writeBytes(redApple + "\n");
			} catch (Exception e){}
		}
	}
}

class PlayedApple {
	public int playerID;
	public String redApple;
	public PlayedApple(int playerID, String redApple) {
		this.playerID = playerID;
		this.redApple = redApple;
	}
}

public class Apples2Apples {
	public ArrayList<String> redApples;
	public ArrayList<String> greenApples;
	public static ArrayList<PlayedApple> playedApple = new ArrayList<PlayedApple>();
	public ArrayList<Player> players = new ArrayList<Player>();
	public Random rnd;

	public static void main(String argv[]) {
		Apples2Apples game;
		if(argv.length == 0) {
			try {
				game = new Apples2Apples(0);				
			} catch (Exception e) {e.printStackTrace(System.out);}
		} else {
			try {
				//If just a number is submitted then this is the Server and there are online clients
				int numberOfOnlineClients = Integer.parseInt(argv[0]);
				game = new Apples2Apples(numberOfOnlineClients);
			} catch(NumberFormatException e) {
				//If it is not a number then we assume it's an URL and then this is one of the online clients
				try {
					game = new Apples2Apples(argv[0]);					
				} catch (Exception err){System.out.println(err.getMessage());}
			} catch(Exception e) {
				e.printStackTrace(System.out);
				System.out.println("Something went wrong");
			}
		}
	}
	/**
	 * This is the constructor when this instance is one of the online clients
	 **/
	public Apples2Apples(String ipAddress) throws Exception {
		//Connect to server
		Socket aSocket = new Socket(ipAddress, 2048);
		DataOutputStream outToServer = new DataOutputStream(aSocket.getOutputStream());
		BufferedReader inFromServer = new BufferedReader(new InputStreamReader(aSocket.getInputStream()));
		//Get the hand of apples from server
		String[] applesString = (inFromServer.readLine()).split(";");
		ArrayList<String> hand = new ArrayList<String>(Arrays.asList(applesString));

		//Setup is completed, now play the game
		while(true) {
			//receive info about being the judge or not
			String judgeString = inFromServer.readLine();
			boolean judge = (judgeString.compareTo("JUDGE")==0);
			//If someone wins the game the FINISHED string is written, and it just happens to be caught here
			if(judgeString.startsWith("FINISHED")) {
				System.out.println("\n"+judgeString);
				break;
			}

			System.out.println("*****************************************************");
			if(judge) {
				System.out.println("**                 NEW ROUND - JUDGE               **");				
			} else {
				System.out.println("**                    NEW ROUND                    **");

			}
			System.out.println("*****************************************************");

			//receive and print the green apple that has been played
			String greenApple = inFromServer.readLine();
			System.out.println(greenApple + "\n");

			if(!judge) {
				//Play your red apple
				System.out.println("Choose a red apple to play");
				for(int i=0; i<hand.size(); i++) {
					System.out.println("["+i+"]   " + hand.get(i));
				}
				System.out.println("");
				int choice;
				BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
				String input=br.readLine();
				choice = Integer.parseInt(input);
				outToServer.writeBytes(hand.get(choice)+"\n");
				hand.remove(choice);
				System.out.println("Waiting for other players\n");				
			}

			//Receive the played apples from server
			String playedApples = (inFromServer.readLine()).replaceAll("#", "\n");
			System.out.println("\nThe following apples were played:\n"+playedApples);

			if(judge) {
				//choose which red apple should win
				System.out.println("Choose which red apple wins\n");
				int choice;
				BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
				String input=br.readLine();
				outToServer.writeBytes(input+"\n");
			}

			String winningRedApple = inFromServer.readLine();
			System.out.println(winningRedApple + "\n");

			//Non judges get a new red apple to replace the one that was played
			if(!judge) {
				String newRedApple = inFromServer.readLine();
				hand.add(newRedApple);
			}
		}
	}

	/**
	 * This is the constructor when this instance is also the server 
	 **/
	public Apples2Apples(int numberOfOnlinePlayers) throws Exception {
  		redApples = new ArrayList<String>(Files.readAllLines(Paths.get("./", "redApples.txt"), StandardCharsets.ISO_8859_1)); 
  		greenApples = new ArrayList<String>(Files.readAllLines(Paths.get("./", "greenApples.txt"), StandardCharsets.ISO_8859_1));

  		//shuffle
		rnd = ThreadLocalRandom.current();
		for(int i=redApples.size()-1; i>0; i--) {
			int index = rnd.nextInt(i+1);
			String a = redApples.get(index); redApples.set(index, redApples.get(i)); redApples.set(i, a); // SWAP
		}
		rnd = ThreadLocalRandom.current();
		for(int i=greenApples.size()-1; i>0; i--) {
			int index = rnd.nextInt(i+1);
			String a = greenApples.get(index); greenApples.set(index, greenApples.get(i)); greenApples.set(i, a); // SWAP
		}

		//Minimum of 4 players, so fill up with bots if numberOfOnlinePlayers is less than 3 (one player is on the server).
		//Make sure that the player on the server is last in the ArrayList of players, or online players can't play

		ServerSocket aSocket = new ServerSocket(2048);
		for(int onlineClient=0; onlineClient<numberOfOnlinePlayers; onlineClient++) {
			Socket connectionSocket = aSocket.accept();
			BufferedReader inFromClient = new BufferedReader(new InputStreamReader(connectionSocket.getInputStream()));
			DataOutputStream outToClient = new DataOutputStream(connectionSocket.getOutputStream());
			String handString = "";
			for(int i=0; i<7; i++) { //Deal 7 cards to the online Player
				handString = ((handString.compareTo("")==0)?"":(handString+";")) + redApples.remove(0); //Create String of Cards, separated by ;
			}
			outToClient.writeBytes(handString+"\n");
			players.add(new Player(onlineClient, false, connectionSocket, inFromClient, outToClient));
			System.out.println("Connected to " + "Player ID: " + (onlineClient));
		}
		//Add bots to reach 3 players excluding server player
		if(numberOfOnlinePlayers < 3) {
			for(int i=numberOfOnlinePlayers; i<=3; i++) {
				players.add(new Player(i, new ArrayList<String>(), true));
				for(int j=0; j<7; j++) { //Deal 7 cards to the Non-online bot
					players.get(i).hand.add(redApples.remove(0));
				}
			}
		}
		//Add server player
		players.add(new Player(players.size(), new ArrayList<String>(), false));
		for(int j=0; j<7; j++) { //Deal 7 cards to the Player
			players.get(players.size()-1).hand.add(redApples.remove(0));
		}

		//Randomise which player starts as judge
		int judge = rnd.nextInt(players.size());

		//****** Setup is completed, start playing the game
		boolean finished = false;
		while(!finished) {

			System.out.println("*****************************************************");
			if(judge==players.size()-1) {
				System.out.println("**                 NEW ROUND - JUDGE               **");				
			} else {
				System.out.println("**                    NEW ROUND                    **");

			}
			System.out.println("*****************************************************");

			String playedGreenApple = greenApples.remove(0);
			//show the green apple to all the online clients and the server player
			System.out.println("Green apple: " + playedGreenApple + "\n");
			for(int i=0; i<numberOfOnlinePlayers; i++) {
				players.get(i).outToClient.writeBytes(((judge==i)?"JUDGE":"NOTJUDGE")+"\n");
				players.get(i).outToClient.writeBytes("Green apple: " + playedGreenApple + "\n");
			}

			//Let all but the judge play their red apple
			//Create a threadpool so all players can pick their red apple at once
			ExecutorService threadpool = Executors.newFixedThreadPool(players.size()-1);	

			for(int i=0; i<players.size(); i++) {
				if(i!=judge) {
					Player currentPlayer = players.get(i);

					//Make sure every player can answer at the same time
					Runnable task = new Runnable() {
						@Override
						public void run() {
							currentPlayer.play();	
						}
					};
					threadpool.execute(task);
				}
			}
			threadpool.shutdown();

			//wait for all the answers to come in
/*			while(playedApple.size() < players.size()-1) {
				Thread.sleep(100);
			}*/
			while(!threadpool.isTerminated()) {
				Thread.sleep(100);
			}

			//Shuffle the answers
			rnd = ThreadLocalRandom.current();
			for(int i=playedApple.size()-1; i>0; i--) {
				int index = rnd.nextInt(i+1);
				PlayedApple a = playedApple.get(index); playedApple.set(index, playedApple.get(i)); playedApple.set(i, a); // SWAP
			}

			//show the played apples to the online players and the server player
			String playedApplesString = playedGreenApple;
			for(int j=0; j<players.size()-1; j++) { //Iterate between the submitted apples
				playedApplesString = playedApplesString + "#\t["+j+"] "+ playedApple.get(j).redApple; //Create printable String of apples
			}
			for(int i=0; i<numberOfOnlinePlayers; i++) {
				players.get(i).outToClient.writeBytes(playedApplesString+"\n");
			}
			playedApplesString = playedApplesString.replaceAll("#", "\n");
			System.out.println("\nThe following apples were played:\n"+playedApplesString);

			//Judge which is the best red apple
			PlayedApple winningApple = players.get(judge).judge();
			players.get(winningApple.playerID).greenApples.add(playedGreenApple); //add point to winner
			
			//notify everyone about who won
			String winnerString = ((players.get(winningApple.playerID).isBot?"Bot":"Player") + " ID"+winningApple.playerID+
										" won with: " + winningApple.redApple);
			System.out.println(winnerString + "\n");
			for(int i=0; i<numberOfOnlinePlayers; i++) {
				players.get(i).outToClient.writeBytes(winnerString+"\n");
			}

			playedApple.clear();
			//refill cards and check winner
			int gameWinner = 0;
			for(int i=0; i<players.size(); i++) {
				if(i!=judge) {
					players.get(i).addCard(redApples.remove(0));
				}
				//Check if any player have enough green apples to win
				if(players.get(i).greenApples.size() >= 4) {
					gameWinner = i;
					finished=true;
				}
			}

			if(finished) {
				String gameWinnerString = "FINISHED: "+((players.get(gameWinner).isBot?"Bot":"Player") + " ID"+gameWinner+
										" won the game");
				//Notify online clients that someone won the game
				for(int i=0; i<numberOfOnlinePlayers; i++) {
					players.get(i).outToClient.writeBytes(gameWinnerString+"\n");
				}
				System.out.println("\n"+gameWinnerString);
			}

			//Assign a new judge
			judge=((judge==(players.size()-1))?0:(judge+1));
		}
	}
}
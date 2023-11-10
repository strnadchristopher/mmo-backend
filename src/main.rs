// Implement a crate in src/Player.rs
// Create a new project with cargo new --lib Player
// Move the Player struct and impl into src/Player.rs
// Add pub to the struct and impl
// Add use Player::Player; to src/main.rs
mod player;
extern crate ws;
use std::sync::{Arc, Mutex};
use ws::listen;
use ws::Message;
use serde_json::{Result, Value};

fn main() {
	let server_socket = "127.0.0.1:2794";
    println!("Hello, world!");
    start_server(server_socket);
}

fn start_server(server_socket: &str){
    println!("Starting server...");
	// Vector of players, defined in player::Player, wrapped in Arc and Mutex for interior mutability
    let players: Arc<Mutex<Vec<player::Player>>> = Arc::new(Mutex::new(Vec::new()));

    // Clone a reference to the Arc for the closure
    let players_clone = Arc::clone(&players);
	// Listen on an address and call the closure for each connection
	if let Err(error) = listen(server_socket, move |out| {
		let players = Arc::clone(&players_clone);
        // The handler needs to take ownership of out, so we use move
        move |msg: Message| {
            // Handle messages received on this connection
            handle_message(msg.clone(), &players, out.clone())
        }
    }) {
        // Inform the user of failure
        println!("Failed to create WebSocket due to {:?}", error);
    }
}

fn handle_message(msg: Message, players: &Arc<Mutex<Vec<player::Player>>>, out: ws::Sender) -> ws::Result<()>{
	println!("Server got message '{}'. ", msg);
	let message_object: Value = serde_json::from_str(msg.as_text().unwrap()).unwrap();

	match message_object["request"].as_str().unwrap(){
		"Connect" =>{
			let player = player::Player::new(message_object["client_id"].to_string());
            players.lock().unwrap().push(player);
			out.send("Player connected")
		},
		"MoveForward" => {
			// Get the player in the players vector with the matching client_id
			let mut player_mutex = players.lock().unwrap();
			let player = player_mutex.iter_mut().find(|player| player.name == message_object["client_id"].to_string()).unwrap();

			// Move the player forward
			player.move_to(player.position.x, player.position.y + 1, player.position.z);
			println!("Player: {:?}", player);
			out.send(format!("Player {} moved forward. Coordinates are now {:?}", player.name, player.position))
		},
		_ => {println!("Unknown request: {}", message_object["request"].as_str().unwrap());
				out.send("Unknown request")}
	}
}

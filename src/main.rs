pub mod prelude;

use std::collections::HashMap;
use prelude::*;
use std::sync::mpsc;
use godot_binary_serialization::types::primitive::GodotString;
use godot_binary_serialization::types::structures::GodotDictionary;
use godot_binary_serialization::types::variant::GodotVariant;
use steamworks::networking_types::NetworkingConfigDataType::String;

const LOBBY_NAME: &str = "Sulayre's Rust Dedicated Server";
const SERVER_VERSION: f32 = 1.1;
const BAN_LIST: &str = "76561199220832861,76561199210086114";

const SERVER_CODE:&str = "SULADSV";

struct ClientWrapper<'w> {
    client:&'w Client,
    single:&'w SingleClient,
}

impl ClientWrapper {
    fn new(client: &Client, single: &SingleClient) -> Self {
        ClientWrapper { client, single }
    }
}


fn main() {
    let (client, single) = Client::init_app(3146520).unwrap();
    let client_wrapper = ClientWrapper::new(&client,&single);

    let matchmaking = client.matchmaking();
    let utils = client.utils();
    let user = client.user();
    let friends = client.friends();
    let networking = client.networking();

    let (sender_create_lobby, receiver_create_lobby) = mpsc::channel();
    let (sender_p2p_request, reciever_p2p_request) = mpsc::channel();

    let mut input_name;
    let mut input_players;

    println!("Insert the lobby list server name.");
    stdin()
        .read_line(&mut input_name)
        .expect("make sure the lobby name is valid!");
    input_name = input_name.trim().to_string(); //there was to be a better way to get rid of the newline

    println!("Insert the lobby max players count.");
    stdin()
        .read_line(&mut input_players)
        .unwrap_or_else(|_| panic!());

    let max_players:u32 = input_players.trim().parse().expect("Given max server players is not a number!");
    if !(2..=250).contains(&max_players) {panic!("Server max players must be higher than 2 and lower than 250!")}
    println!(
        "Attempting to host a WEBFISHING ({:?}) Dedicated Server under the steam user {:?} ({:?})",
        utils.app_id().0,
        friends.name(),
        user.steam_id().raw(),
    );
    println!(
        "Lobby Name: {:?}",
        input_name
    );
    println!(
        "Max Players: {:?}",
        max_players
    );

    matchmaking.create_lobby(LobbyType::Private, max_players, move |result| match result {
        Ok(lobby_id) => {
            sender_create_lobby.send(lobby_id).unwrap();
        }
        Err(err) => panic!("Error: {}", err),
    });

    client.register_callback(move |message: P2PSessionRequest| {
        println!("Lobby chat message received: {:?}", message);
        sender_p2p_request.send(message).unwrap();
    });

    let mut lobby_id:LobbyId = LobbyId(0);

    loop {
        single.run_callbacks();
        if let Ok(id) = receiver_create_lobby.try_recv(){
            println!("Setting lobby data");
            lobby_id = id;
            matchmaking.set_lobby_data(lobby_id, "name", "Server");
            matchmaking.set_lobby_data(lobby_id, "name", LOBBY_NAME);
            matchmaking.set_lobby_data(lobby_id, "ref", "webfishing_gamelobby");
            matchmaking.set_lobby_data(lobby_id, "version", SERVER_VERSION.to_string().as_str());
            matchmaking.set_lobby_data(lobby_id, "code", SERVER_CODE);
            matchmaking.set_lobby_data(lobby_id, "type", "public");
            matchmaking.set_lobby_data(lobby_id, "age_limit", "false");
            matchmaking.set_lobby_data(lobby_id, "public", "true");
            matchmaking.set_lobby_data(lobby_id, "cap", max_players.to_string().as_str());
            matchmaking.set_lobby_data(lobby_id, "banned_players", BAN_LIST);
            matchmaking.set_lobby_data(lobby_id, "server_browser_value", "0");
        }
        if let Ok(request) = reciever_p2p_request.try_recv(){
            let user_id = request.remote;
            let members = matchmaking.lobby_members(lobby_id);
            if !members.contains(&user_id) {
                println!("P2P Request sender is no longer connected!");
            }
            networking.accept_p2p_session(user_id);
            println!("Accepted P2P Request from {:?}!",user_id);

            let rust_packet = indexmap! {
                Box::new(GodotString::new("type")) => Box::new(GodotString::new("hadnshake"))
            };
            
            let godot_packet = GodotDictionary::new_from_map(rust_packet).bytes();

            for player in members {
                println!("Sending P2P Handshake to {:?}",player);
                networking.send_p2p_packet(player,SendType::Reliable, &godot_packet);
            }
        }
    }
}
pub mod prelude;
pub mod structs;
mod lobby_handler;

use prelude::*;
use std::sync::mpsc;
use steamworks::networking_types::NetworkingConfigDataType::String;

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

    let (sender_create_lobby, receiver_create_lobby) = mpsc::channel();
    let (sender_lobby_chat_msg, receiver_lobby_chat_msg) = mpsc::channel();

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

    lobby_handler::create_lobby(&client_wrapper,max_players);

    loop {
        single.run_callbacks();
        if let Ok(lobby_id) = receiver_create_lobby.try_recv() {
            println!("Sending message to lobby chat...");
            matchmaking
                .send_lobby_chat_message(lobby_id, &[0, 1, 2, 3, 4, 5])
                .expect("Failed to send chat message to lobby");
        }
    }
}
use std::sync::mpsc;
use crate::ClientWrapper;
use crate::prelude::*;

const LOBBY_NAME: &str = "Sulayre's Rust Dedicated Server";
const SERVER_VERSION: f32 = 1.1;
const BAN_LIST: &str = "76561199220832861,76561199210086114";

const SERVER_CODE:&str = "SULADSV";

pub fn create_lobby(client_wrapper: &ClientWrapper, max_players:u32) {
    let client = &client_wrapper.client;
    let single = &client_wrapper.single;

    let matchmaking = client.matchmaking();

    let mut lobby_id: LobbyId = LobbyId(0);

    matchmaking.create_lobby(LobbyType::Private, max_players, move |result| match result {
        Ok(id) => {
            lobby_id = id
        }
        Err(err) => panic!("Error: {}", err),
    });
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
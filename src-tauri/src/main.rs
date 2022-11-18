#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use lazy_static::lazy_static;
use reqwest::Client;

lazy_static! {
    static ref CLIENT: Client = Client::new();
}

#[tauri::command]
async fn user_auth() -> bool {
    let res = CLIENT
        .get("https://api.spotify.com/authorize")
        .query(&[
            ("client_id", "100d6e78724442679cfd24b391b63445"),
            ("response_type", "code"),
            ("redirect_uri", "http://localhost:1420/callback"),
            ("state", "a"), //TODO: generate random state
            ("code_challenge_method", "S256"),
            ("code_challenge", "a"), //TODO: generate code challenge
            ("scope", "user-library-read user-read-playback-state user-read-currently-playing playlist-read-collaborative playlist-read-private user-modify-playback-state"),
        ])
        .send().await.unwrap();

    if res.status().is_success() {
        return true;
    }

    false
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![user_auth])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

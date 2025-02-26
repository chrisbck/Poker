mod card_dealer;
mod game_controller;
mod poker_hand;
mod player;
mod table;
mod api; // New module for API

use warp::Filter;
use std::sync::{Arc, Mutex};
use game_controller::GameController;
use api::{AppState, get_routes};

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        game_controller: Mutex::new(GameController::new()),
    });

    // Initialize players
    {
        let mut controller = state.game_controller.lock().unwrap();
        controller.initialize_players(vec![
            ("1".to_string(), "Alice".to_string(), 0, 1000),
            ("2".to_string(), "Bob".to_string(), 1, 1000),
            ("3".to_string(), "Charlie".to_string(), 2, 1000),
        ]);
    }

    // Start the server with refactored routes
    warp::serve(get_routes(state))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

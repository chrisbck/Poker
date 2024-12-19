mod card_dealer;
mod game_controller;
mod poker_hand;

use warp::{Filter, http::header::HeaderMap};
use std::sync::{Arc, Mutex};
use game_controller::GameController;

pub struct AppState {
    game_controller: Mutex<GameController>,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        game_controller: Mutex::new(GameController::new()),
    });

    // Route to deal hole cards
    let deal_hole_route = warp::path("deal_hole")
    .and(with_state(state.clone()))
    .map(move |state: Arc<AppState>| {
        let mut controller = state.game_controller.lock().unwrap();

        if let Some(cards) = controller.deal_hole_cards() {
            // Standardized response with "type" and "cards"
            warp::reply::json(&serde_json::json!({
                "type": "hole",
                "cards": cards
            }))
        } else {
            warp::reply::json(&serde_json::json!({
                "type": "error",
                "message": "Not enough cards in the deck"
            }))
        }
    });

    // Route to deal all community cards (flop, turn, river)
    let deal_community_route = warp::path("deal_community")
    .and(with_state(state.clone()))
    .map(move |state: Arc<AppState>| {
        let mut controller = state.game_controller.lock().unwrap();

        if let Some(cards) = controller.deal_community_cards() {
            // Standardized response with "type" and "cards"
            warp::reply::json(&serde_json::json!({
                "type": "community",
                "cards": cards
            }))
        } else {
            warp::reply::json(&serde_json::json!({
                "type": "error",
                "message": "Not enough cards in the deck"
            }))
        }
    });


    // Route to reset the game state
    let reset_route = warp::path("reset")
    .and(with_state(state.clone()))
    .map(move |state: Arc<AppState>| {
        let mut controller = state.game_controller.lock().unwrap();
        controller.reset_deck();

        warp::reply::json(&serde_json::json!({
            "type": "reset",
            "message": "Game Reset Successfully"
        }))
    });


    // Route to get the best hand evaluation
    let evaluate_route = warp::path("evaluate")
    .and(with_state(state.clone()))
    .map(move |state: Arc<AppState>| {
        let controller = state.game_controller.lock().unwrap();

        if let Some(best_hand) = controller.get_last_evaluated_hand() {
            warp::reply::json(&serde_json::json!({
                "type": "evaluation",
                "best_hand": best_hand
            }))
        } else {
            warp::reply::json(&serde_json::json!({
                "type": "error",
                "message": "Insufficient cards to evaluate hand"
            }))
        }
    });


    // Combine all routes
    let routes = deal_hole_route
        .or(deal_community_route)
        .or(reset_route)
        .or(evaluate_route);

    // Start the server
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// Helper function to share state between routes
fn with_state(
    state: Arc<AppState>,
) -> impl Filter<Extract = (Arc<AppState>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

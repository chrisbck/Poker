mod card_dealer;
mod game_controller;
mod poker_hand;
mod player;
mod table;

use warp::Filter;
use std::{collections::HashMap, sync::{Arc, Mutex}};
use game_controller::GameController;

pub struct AppState {
    game_controller: Mutex<GameController>,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        game_controller: Mutex::new(GameController::new()),
    }); // <-- Closing brace added here!
    
    // Initialize players (replace with a dynamic setup if needed)
    {
        let mut controller = state.game_controller.lock().unwrap();
        controller.initialize_players(vec![
            ("1".to_string(), "Alice".to_string(), 0, 1000),
            ("2".to_string(), "Bob".to_string(), 1, 1000),
            ("3".to_string(), "Charlie".to_string(), 2, 1000),
        ]);
    }
    

    // Route to deal hole cards to all players
    let deal_hole_route = warp::path("deal_hole")
        .and(with_state(state.clone()))
        .map(move |state: Arc<AppState>| {
            let mut controller = state.game_controller.lock().unwrap();

            match controller.deal_hole_cards() {
                Ok(_) => {
                    let player_cards: Vec<_> = controller
                        .get_players()
                        .iter()
                        .map(|player| serde_json::json!({
                            "name": player.display_name,
                            "hole_cards": player.hole_cards
                        }))
                        .collect();

                    warp::reply::json(&serde_json::json!({
                        "type": "hole",
                        "players": player_cards
                    }))
                }
                Err(err) => warp::reply::json(&serde_json::json!({
                    "type": "error",
                    "message": err
                })),
            }
        });

    // Route to deal community cards
    let deal_community_route = warp::path("deal_community")
        .and(with_state(state.clone()))
        .map(move |state: Arc<AppState>| {
            let mut controller = state.game_controller.lock().unwrap();

            match controller.deal_community_cards() {
                Ok(_) => warp::reply::json(&serde_json::json!({
                    "type": "community",
                    "cards": controller.get_community_cards()
                })),
                Err(err) => warp::reply::json(&serde_json::json!({
                    "type": "error",
                    "message": err
                })),
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

    // Route to evaluate all player hands
    let evaluate_route = warp::path("evaluate")
        .and(with_state(state.clone()))
        .map(move |state: Arc<AppState>| {
            let controller = state.game_controller.lock().unwrap();

            let player_hands: Vec<_> = controller.get_players().iter().map(|player| {
                serde_json::json!({
                    "name": player.display_name,
                    "hand_strength": player.hand_strength,
                    "best_hand": player.best_hand,
                    "hole_cards": player.hole_cards
                })
            }).collect();

            warp::reply::json(&serde_json::json!({
                "type": "evaluation",
                "players": player_hands,
                "community_cards": controller.get_community_cards()
            }))
        });

        
    let test_winners_route = warp::path("test_winners")
    .and(with_state(state.clone()))
    .map(move |state: Arc<AppState>| {
        let mut controller = state.game_controller.lock().unwrap();

        // Ensure all hands are evaluated first
        controller.evaluate_player_hands();

        // Get all player IDs from the game
        let all_players: Vec<String> = controller.get_players()
            .iter()
            .map(|p| p.player_id.clone())
            .collect();

        match controller.get_winners(&all_players) {
            Some(winners) => {
                let winner_list: Vec<_> = winners.iter().map(|player_id| {
                    let player = controller.get_players().iter()
                        .find(|p| &p.player_id == player_id)
                        .unwrap(); // Safe unwrap, since we just got these IDs from `get_players()`

                    serde_json::json!({
                        "player_id": player_id,
                        "name": player.display_name,
                        "hand_strength": player.hand_strength,
                        "best_hand": player.best_hand.as_ref().map(|h| h.cards.clone())
                    })
                }).collect();

                warp::reply::json(&serde_json::json!({
                    "type": "test_winners",
                    "players": winner_list
                }))
            }
            None => warp::reply::json(&serde_json::json!({
                "type": "error",
                "message": "No winner determined"
            })),
        }
    });
    
    

    // Combine all routes
    let routes = deal_hole_route
        .or(deal_community_route)
        .or(reset_route)
        .or(evaluate_route)
        .or(test_winners_route);

    // Start the server
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// Helper function to share state between routes
fn with_state(
    state: Arc<AppState>,
) -> impl Filter<Extract = (Arc<AppState>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

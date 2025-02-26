use warp::Filter;
use std::{sync::{Arc, Mutex}};
use crate::game_controller::GameController;

/// Struct representing the shared state of the application.
/// Contains a `GameController` wrapped in a `Mutex` for thread safety.
pub struct AppState {
    pub game_controller: Mutex<GameController>,
}

/// Helper function to create a Warp filter for sharing the application state.
///
/// This function ensures that all routes can access the game state safely.
///
/// # Arguments
/// * `state` - An `Arc<AppState>` that holds the game controller.
///
/// # Returns
/// A `warp::Filter` that provides the shared state.
fn with_state(
    state: Arc<AppState>,
) -> impl Filter<Extract = (Arc<AppState>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

/// API route to deal hole cards to all players.
///
/// This endpoint assigns two hole cards to each player.
///
/// # Endpoint
/// `GET /deal_hole`
///
/// # Response
/// - **Success**: Returns a JSON object containing each player's hole cards.
/// - **Failure**: Returns an error message if cards cannot be dealt.
fn deal_hole_route(state: Arc<AppState>) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("deal_hole")
        .and(with_state(state))
        .map(|state: Arc<AppState>| {
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
        })
}

/// API route to deal community cards.
///
/// This endpoint assigns five community cards to the table.
///
/// # Endpoint
/// `GET /deal_community`
///
/// # Response
/// - **Success**: Returns a JSON object with the community cards.
/// - **Failure**: Returns an error message if cards cannot be dealt.
fn deal_community_route(state: Arc<AppState>) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("deal_community")
        .and(with_state(state))
        .map(|state: Arc<AppState>| {
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
        })
}

/// API route to reset the game state.
///
/// This endpoint resets the deck and clears all game state.
///
/// # Endpoint
/// `GET /reset`
///
/// # Response
/// - **Success**: Returns a confirmation message.
fn reset_route(state: Arc<AppState>) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("reset")
        .and(with_state(state))
        .map(|state: Arc<AppState>| {
            let mut controller = state.game_controller.lock().unwrap();
            controller.reset_deck();
            warp::reply::json(&serde_json::json!({
                "type": "reset",
                "message": "Game Reset Successfully"
            }))
        })
}

/// API route to evaluate all player hands.
///
/// This endpoint calculates the best possible hand for each player.
///
/// # Endpoint
/// `GET /evaluate`
///
/// # Response
/// - **Success**: Returns each player's best hand and strength.
fn evaluate_route(state: Arc<AppState>) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("evaluate")
        .and(with_state(state))
        .map(|state: Arc<AppState>| {
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
        })
}

/// API route to determine the winner(s).
///
/// This endpoint identifies the best hand(s) among all players.
///
/// # Endpoint
/// `GET /test_winners`
///
/// # Response
/// - **Success**: Returns the winning player(s) and their best hand.
/// - **Failure**: Returns an error message if no winner is found.
fn test_winners_route(state: Arc<AppState>) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("test_winners")
        .and(with_state(state))
        .map(|state: Arc<AppState>| {
            let controller = state.game_controller.lock().unwrap();
            let all_players: Vec<String> = controller.get_players()
                .iter()
                .map(|p| p.player_id.clone())
                .collect();

            match controller.get_winners(&all_players) {
                Some(winners) => {
                    let winner_list: Vec<_> = winners.iter().map(|player_id| {
                        let player = controller.get_players().iter()
                            .find(|p| &p.player_id == player_id)
                            .unwrap();
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
        })
}

/// Combines all API routes into a single filter.
///
/// This function collects all endpoints and allows them to be served
/// from the main application.
///
/// # Arguments
/// * `state` - The shared game state (`Arc<AppState>`).
///
/// # Returns
/// A `warp::Filter` containing all defined routes.
pub fn get_routes(state: Arc<AppState>) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    deal_hole_route(state.clone())
        .or(deal_community_route(state.clone()))
        .or(reset_route(state.clone()))
        .or(evaluate_route(state.clone()))
        .or(test_winners_route(state.clone()))
}

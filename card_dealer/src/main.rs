mod card_dealer; // Declare the card_dealer module
mod game_controller;

use warp::Filter;
use std::sync::{Arc, Mutex};
use game_controller::GameController;


pub struct PlayerGateway{
    game_controller: Arc<Mutex<GameController>>,    // A thread safe instance of a GameController
}

impl PlayerGateway{
    pub fn new() -> Self{
        Self{
            game_controller: Arc::new(Mutex::new(GameController::new())),
        }
    }

    pub fn deal(&self, count: usize) -> Option<Vec<crate::card_dealer::Card>>{
        let mut controller = self.game_controller.lock().unwrap();
        controller.deal(count)
    }
}



#[tokio::main]
async fn main() {
    // Create the PlayerGateway
    let gateway = Arc::new(PlayerGateway::new());

    // Define the /deal endpoint
    let deal_route = warp::path("deal")
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .and(with_gateway(gateway.clone()))
        .map(|params: std::collections::HashMap<String, String>, gateway: Arc<PlayerGateway>| {
            let count = params
                .get("count")
                .and_then(|c| c.parse::<usize>().ok())
                .unwrap_or(5);

            if let Some(cards) = gateway.deal(count) {
                warp::reply::json(&cards)
            } else {
                warp::reply::json(&"Not enough cards")
            }
        });

    // Start the server
    warp::serve(deal_route).run(([127, 0, 0, 1], 3030)).await;
}

// Helper function to pass the PlayerGateway to Warp
fn with_gateway( gateway: Arc<PlayerGateway>,) ->
impl Filter<Extract = (Arc<PlayerGateway>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || gateway.clone())
}

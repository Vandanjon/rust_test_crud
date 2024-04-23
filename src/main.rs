mod data;
mod db;
mod error;
mod handler;

use warp::{http::StatusCode, Filter};

#[tokio::main]
async fn main() {
    let health_route = warp::path!("health").map(|| warp::reply::with_status("OK", StatusCode::OK));

    // copilot
    // let routes = health_route.or(handler::get_all_users()).or(handler::get_user_by_id()).or(handler::create_user()).or(handler::update_user()).or(handler::delete_user());

    let routes = health_route.with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

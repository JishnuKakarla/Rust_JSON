pub mod handler;
pub mod models;

use handler::get_users;
use handler::add_user;
use handler::get_user_by_id;
use axum::{Router};
use axum::routing::post;
use axum::routing::get;
use dotenv::dotenv;


#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .route("/getjson", get(get_users))
        .route("/postjson", post(add_user))
        .route("/getbyId/:?id", get(get_user_by_id));
    
    println!("Server started successfully");
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}


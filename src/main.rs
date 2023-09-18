use axum::{Router};
use axum::routing::post;
use axum::routing::get;
use dotenv::dotenv;

mod controllers;
mod models;


#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .route("/getjson", get(controllers::auth::get_users))
        .route("/postjson", post(controllers::auth::add_user))
        .route("/getbyId/:?id", get(controllers::auth::get_user_by_id));
    
    println!("Server started successfully");
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}


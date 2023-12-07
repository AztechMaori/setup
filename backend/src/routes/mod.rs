use axum::{routing::get, Router, http::Method};
use tower_http::cors::{CorsLayer, Any};

//hello world 
mod hello_world;
use  hello_world::hello_world;

// JSON
mod JSON;
use JSON::{car_details,return_json};

// SeaORM 
mod db_query;
use db_query::query_data; 






pub fn create_routes() -> Router {

    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_origin(Any);

    return Router::new()
    .route("/", get(|| async { "wassup,bitches!" }))
    .route("/route", get(hello_world))
    .route("/json", get(return_json))
    .route("/car", get(car_details))
    .layer(cors);

    
}
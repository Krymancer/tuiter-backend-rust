mod models;
mod routes;
mod router;
mod repository;

use router::get_router;

#[tokio::main]
async fn main() {
    let app = get_router();

    axum::Server::bind(&"0.0.0.0:1234".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

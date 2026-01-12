use axum::{routing::get, Router};
use lcn::api::{hostinfo, scanhosts};
use lcn::LCN_PORT;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    println!("Starting LCN server on port {}...", LCN_PORT);

    let app = Router::new()
        .route("/hostinfo", get(hostinfo))
        .route("/scanhosts", get(scanhosts));

    let addr = SocketAddr::from(([0, 0, 0, 0], LCN_PORT));
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}

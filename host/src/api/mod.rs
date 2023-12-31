mod routers;

use axum::Router;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use routers::{
    hello::hello_router,
    prover::prover_router,
    verifier::verifier_router,
    ai::ai_router,
};


pub async fn api_start() {
    let api_routes = Router::new()
        .nest("/hello", hello_router())
        .nest("/prover", prover_router())
        .nest("/verifier", verifier_router())
        .nest("/ai", ai_router());

    let addr = SocketAddr::from((
        IpAddr::V4(Ipv4Addr::LOCALHOST),
        3000
    ));
    println!("listening on {}", addr);
    
    let app = Router::new().nest("/", api_routes);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

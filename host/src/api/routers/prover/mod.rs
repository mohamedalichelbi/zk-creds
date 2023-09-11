mod rhai_handlers;

use axum::routing::{Router, post};
use rhai_handlers::gen_rhai_proof;


pub fn prover_router() -> Router {
    Router::new()
        .nest("/genproof", genproof_router())
}


fn genproof_router() -> Router {
    Router::new()
        .route("/rhai", post(gen_rhai_proof))
}

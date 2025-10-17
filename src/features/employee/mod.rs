pub mod handler;
pub mod logic;
pub mod repo;

use std::sync::Arc;

use axum::Router;
use axum::routing::post;

use crate::features::employee::handler::Handler;
use crate::features::employee::logic::Logic;
use crate::features::employee::repo::Repo;

pub fn new(pool: &sqlx::PgPool) -> Router {
    let pool = Arc::new(pool.clone());
    let repo = Arc::new(Repo::new(pool));
    let logic = Arc::new(Logic::new(repo));
    let handler = Arc::new(Handler::new(logic));

    Router::new()
        .route("/employee", post(Handler::create))
        .with_state(handler)
}

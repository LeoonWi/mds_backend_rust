use std::sync::Arc;

use axum::{Router, routing::get};

use crate::features::services::{handler::Handler, logic::Logic, repo::Repo};

pub mod handler;
pub mod logic;
pub mod repo;

pub fn new(pool: &sqlx::PgPool) -> Router {
    let pool = Arc::new(pool.clone());
    let repo = Arc::new(Repo::new(pool));
    let logic = Arc::new(Logic::new(repo));
    let handler = Arc::new(Handler::new(logic));

    Router::new()
        .route("/create_service", get(Handler::create_service))
        .with_state(handler)
}

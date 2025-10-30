//! REST APIハンドラー
//!
//! エージェント登録、ヘルスチェック、プロキシAPI

pub mod agent;
pub mod health;

use axum::{
    routing::post,
    Router,
};
use crate::AppState;

/// APIルーターを作成
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/api/agents", post(agent::register_agent))
        .route("/api/health", post(health::health_check))
        .with_state(state)
}

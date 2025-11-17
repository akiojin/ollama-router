#![allow(dead_code)]

use std::net::SocketAddr;

use ollama_coordinator_coordinator::{
    api, balancer::LoadManager, registry::AgentRegistry, tasks::DownloadTaskManager, AppState,
};
use reqwest::{Client, Response};
use serde_json::json;

use super::http::{spawn_router, TestServer};

/// テスト用のインメモリSQLiteデータベースプールを作成
pub async fn create_test_db_pool() -> sqlx::SqlitePool {
    let pool = sqlx::SqlitePool::connect("sqlite::memory:")
        .await
        .expect("Failed to create in-memory database");

    // マイグレーションを実行
    ollama_coordinator_coordinator::db::migrations::run_migrations(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

/// テスト用のJWT秘密鍵を返す
pub fn test_jwt_secret() -> String {
    "test-secret-key-for-testing-only-do-not-use-in-production".to_string()
}

/// コーディネーターサーバーをテスト用に起動する
pub async fn spawn_coordinator() -> TestServer {
    // AUTH_DISABLED=trueで認証を無効化（既存テスト互換性のため）
    std::env::set_var("AUTH_DISABLED", "true");

    let registry = AgentRegistry::new();
    let load_manager = LoadManager::new(registry.clone());
    let request_history = std::sync::Arc::new(
        ollama_coordinator_coordinator::db::request_history::RequestHistoryStorage::new().unwrap(),
    );
    let task_manager = DownloadTaskManager::new();
    let db_pool = create_test_db_pool().await;
    let jwt_secret = test_jwt_secret();

    let state = AppState {
        registry,
        load_manager,
        request_history,
        task_manager,
        db_pool,
        jwt_secret,
    };

    let router = api::create_router(state);
    spawn_router(router).await
}

/// 指定したコーディネーターにエージェントを登録する
pub async fn register_agent(
    coordinator_addr: SocketAddr,
    agent_addr: SocketAddr,
) -> reqwest::Result<Response> {
    Client::new()
        .post(format!("http://{coordinator_addr}/api/agents"))
        .json(&json!({
            "machine_name": "stub-agent",
            "ip_address": agent_addr.ip().to_string(),
            "ollama_version": "0.0.0-test",
            "ollama_port": agent_addr.port(),
            "gpu_available": true,
            "gpu_devices": [
                {"model": "Test GPU", "count": 1}
            ]
        }))
        .send()
        .await
}

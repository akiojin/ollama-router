use std::net::SocketAddr;

use or_router::{
    api, balancer::LoadManager, registry::NodeRegistry, tasks::DownloadTaskManager, AppState,
};
use reqwest::{Client, Response};
use serde_json::json;

use super::http::{spawn_router, TestServer};

/// ルーターサーバーをテスト用に起動する
pub async fn spawn_test_router() -> TestServer {
    // テスト用に一時ディレクトリを設定
    let temp_dir = std::env::temp_dir().join(format!("or-test-{}", std::process::id()));
    std::fs::create_dir_all(&temp_dir).unwrap();
    std::env::set_var("OLLAMA_ROUTER_DATA_DIR", &temp_dir);

    let registry = NodeRegistry::new();
    let load_manager = LoadManager::new(registry.clone());
    let request_history =
        std::sync::Arc::new(or_router::db::request_history::RequestHistoryStorage::new().unwrap());
    let task_manager = DownloadTaskManager::new();
    let state = AppState {
        registry,
        load_manager,
        request_history,
        task_manager,
    };

    let router = api::create_router(state);
    spawn_router(router).await
}

/// 指定したルーターにノードを登録する
pub async fn register_node(
    router_addr: SocketAddr,
    node_addr: SocketAddr,
) -> reqwest::Result<Response> {
    Client::new()
        .post(format!("http://{router_addr}/api/nodes"))
        .json(&json!({
            "machine_name": "stub-node",
            "ip_address": node_addr.ip().to_string(),
            "ollama_version": "0.0.0-test",
            // ノードAPIポートは ollama_port+1 という前提のため、APIポートから1引いた値を報告する
            "ollama_port": node_addr.port().saturating_sub(1),
            "gpu_available": true,
            "gpu_devices": [
                {"model": "Test GPU", "count": 1}
            ]
        }))
        .send()
        .await
}

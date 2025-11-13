//! モデル管理APIハンドラー
//!
//! コーディネーターからのモデルプル要求を受信・処理

use crate::ollama::OllamaManager;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info};

/// アプリケーション状態（Ollamaマネージャー）
#[derive(Clone)]
pub struct AppState {
    /// Ollamaマネージャー（モデルプル処理用）
    pub ollama_manager: Arc<Mutex<OllamaManager>>,
}

/// モデルプルリクエスト
#[derive(Debug, Deserialize)]
pub struct PullModelRequest {
    /// プルするモデル名
    pub model: String,
}

/// モデルプルレスポンス
#[derive(Debug, Serialize)]
pub struct PullModelResponse {
    /// 成功メッセージ
    pub message: String,
    /// プルしたモデル名
    pub model: String,
}

/// POST /pull - コーディネーターからのモデルプル要求
pub async fn pull_model(
    State(state): State<AppState>,
    Json(request): Json<PullModelRequest>,
) -> Result<Json<PullModelResponse>, AppError> {
    let model = request.model;

    info!("Received model pull request: {}", model);

    // Ollamaマネージャーを取得
    let ollama_manager = state.ollama_manager.lock().await;

    // モデルをプル（既に存在する場合はスキップ）
    ollama_manager
        .ensure_model(&model)
        .await
        .map_err(|e| AppError(format!("Failed to pull model {}: {}", model, e)))?;

    info!("Model pull completed: {}", model);

    Ok(Json(PullModelResponse {
        message: "Model pulled successfully".to_string(),
        model,
    }))
}

/// Axum用のエラーレスポンス型
#[derive(Debug)]
pub struct AppError(
    /// エラーメッセージ
    String,
);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        error!("API error: {}", self.0);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": self.0
            })),
        )
            .into_response()
    }
}

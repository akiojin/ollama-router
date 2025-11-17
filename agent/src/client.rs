//! Coordinator通信クライアント
//!
//! エージェント登録とハートビート送信

use ollama_coordinator_common::{
    error::{AgentError, AgentResult},
    protocol::{HealthCheckRequest, RegisterRequest, RegisterResponse},
};
use std::fs;
use std::path::PathBuf;
use tracing::info;
use uuid::Uuid;

/// Coordinatorクライアント
pub struct CoordinatorClient {
    coordinator_url: String,
    agent_id: Option<Uuid>,
    agent_token: Option<String>, // T088: エージェント認証トークン
    http_client: reqwest::Client,
}

impl CoordinatorClient {
    /// 新しいCoordinatorクライアントを作成
    pub fn new(coordinator_url: String) -> Self {
        // T089: 既存のトークンファイルがあれば読み込む
        let agent_token = Self::load_token().ok();

        Self {
            coordinator_url,
            agent_id: None,
            agent_token,
            http_client: reqwest::Client::new(),
        }
    }

    /// トークンファイルのパスを取得
    fn token_file_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".ollama-agent")
            .join("token")
    }

    /// トークンをファイルから読み込む（T089）
    fn load_token() -> AgentResult<String> {
        let token_path = Self::token_file_path();

        if !token_path.exists() {
            return Err(AgentError::CoordinatorConnection(
                "Token file not found".to_string(),
            ));
        }

        let token = fs::read_to_string(&token_path)
            .map_err(|e| {
                AgentError::CoordinatorConnection(format!("Failed to read token file: {}", e))
            })?
            .trim()
            .to_string();

        Ok(token)
    }

    /// トークンをファイルに保存する（T089）
    fn save_token(token: &str) -> AgentResult<()> {
        let token_path = Self::token_file_path();

        // ディレクトリが存在しない場合は作成
        if let Some(parent) = token_path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                AgentError::CoordinatorConnection(format!(
                    "Failed to create token directory: {}",
                    e
                ))
            })?;
        }

        // トークンを保存
        fs::write(&token_path, token).map_err(|e| {
            AgentError::CoordinatorConnection(format!("Failed to save token file: {}", e))
        })?;

        info!("Agent token saved to {:?}", token_path);
        Ok(())
    }

    /// エージェントを登録
    pub async fn register(&mut self, req: RegisterRequest) -> AgentResult<RegisterResponse> {
        let url = format!("{}/api/agents", self.coordinator_url);

        // T090: X-Agent-Tokenヘッダーを追加（既存のトークンがある場合）
        let mut request_builder = self.http_client.post(&url).json(&req);
        if let Some(ref token) = self.agent_token {
            request_builder = request_builder.header("X-Agent-Token", token);
        }

        let response = request_builder
            .send()
            .await
            .map_err(|e| AgentError::CoordinatorConnection(format!("Failed to register: {}", e)))?;

        if !response.status().is_success() {
            return Err(AgentError::Registration(format!(
                "Registration failed with status: {}",
                response.status()
            )));
        }

        let register_response = response
            .json::<RegisterResponse>()
            .await
            .map_err(|e| AgentError::Registration(format!("Failed to parse response: {}", e)))?;

        // エージェントIDを保存
        self.agent_id = Some(register_response.agent_id);

        // T088: エージェントトークンを抽出して保存（T089）
        if let Some(ref token) = register_response.agent_token {
            Self::save_token(token)?;
            self.agent_token = Some(token.clone());
            info!("Agent token received and saved");
        }

        info!(
            "Registered with Coordinator: agent_id = {}",
            register_response.agent_id
        );

        Ok(register_response)
    }

    /// ヘルスチェックを送信
    pub async fn send_heartbeat(&self, req: HealthCheckRequest) -> AgentResult<()> {
        let url = format!("{}/api/health", self.coordinator_url);

        // T090: X-Agent-Tokenヘッダーを追加
        let mut request_builder = self.http_client.post(&url).json(&req);
        if let Some(ref token) = self.agent_token {
            request_builder = request_builder.header("X-Agent-Token", token);
        }

        let response = request_builder
            .send()
            .await
            .map_err(|e| AgentError::Heartbeat(format!("Failed to send heartbeat: {}", e)))?;

        if !response.status().is_success() {
            return Err(AgentError::Heartbeat(format!(
                "Heartbeat failed with status: {}",
                response.status()
            )));
        }

        Ok(())
    }

    /// 保存されたエージェントIDを取得
    pub fn get_agent_id(&self) -> Option<Uuid> {
        self.agent_id
    }

    /// 利用中のコーディネーターURLを取得
    pub fn coordinator_url(&self) -> &str {
        &self.coordinator_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinator_client_creation() {
        let client = CoordinatorClient::new("http://localhost:8080".to_string());
        assert_eq!(client.coordinator_url, "http://localhost:8080");
        assert!(client.agent_id.is_none());
    }

    #[test]
    fn test_get_agent_id_none() {
        let client = CoordinatorClient::new("http://localhost:8080".to_string());
        assert!(client.get_agent_id().is_none());
    }
}

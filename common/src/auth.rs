// T032-T037: 認証関連のデータモデル（最小実装、テスト用）

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// ユーザーロール
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Viewer,
}

/// ユーザー
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

/// APIキー
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: Uuid,
    pub key_hash: String,
    pub name: String,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// APIキー（平文付き、発行時のレスポンス用）
#[derive(Debug, Clone, Serialize)]
pub struct ApiKeyWithPlaintext {
    pub id: Uuid,
    pub key: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// エージェントトークン
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentToken {
    pub agent_id: Uuid,
    pub token_hash: String,
    pub created_at: DateTime<Utc>,
}

/// エージェントトークン（平文付き、発行時のレスポンス用）
#[derive(Debug, Clone, Serialize)]
pub struct AgentTokenWithPlaintext {
    pub agent_id: Uuid,
    pub token: String,
    pub created_at: DateTime<Utc>,
}

/// JWTクレーム
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,      // ユーザーID
    pub role: UserRole,   // ロール
    pub exp: usize,       // 有効期限（Unix timestamp）
}

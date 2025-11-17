// T044-T046: JWT生成と検証（スタブ実装、テスト用）

use ollama_coordinator_common::auth::{Claims, UserRole};
use ollama_coordinator_common::error::CoordinatorError;

/// JWTトークンを生成
pub fn create_jwt(
    _user_id: &str,
    _role: UserRole,
    _secret: &str,
) -> Result<String, CoordinatorError> {
    // スタブ実装: テストがREDになるように空の実装
    unimplemented!("create_jwt not yet implemented")
}

/// JWTトークンを検証
pub fn verify_jwt(_token: &str, _secret: &str) -> Result<Claims, CoordinatorError> {
    // スタブ実装: テストがREDになるように空の実装
    unimplemented!("verify_jwt not yet implemented")
}

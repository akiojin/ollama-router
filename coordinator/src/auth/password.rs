// T042-T043: パスワードハッシュ化と検証（スタブ実装、テスト用）

use ollama_coordinator_common::error::CoordinatorError;

/// パスワードをbcryptでハッシュ化
pub fn hash_password(_password: &str) -> Result<String, CoordinatorError> {
    // スタブ実装: テストがREDになるように空の実装
    unimplemented!("hash_password not yet implemented")
}

/// パスワードを検証
pub fn verify_password(_password: &str, _hash: &str) -> Result<bool, CoordinatorError> {
    // スタブ実装: テストがREDになるように空の実装
    unimplemented!("verify_password not yet implemented")
}

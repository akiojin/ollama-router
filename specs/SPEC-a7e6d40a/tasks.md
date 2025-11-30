# タスク: CLI インターフェース整備

**入力**: `/specs/SPEC-a7e6d40a/`の設計ドキュメント
**前提条件**: plan.md (必須)

## フォーマット: `[ID] [P?] 説明`

- **[P]**: 並列実行可能 (異なるファイル、依存関係なし)
- 説明には正確なファイルパスを含める

## Phase 3.1: セットアップ

- [ ] T001 `router/Cargo.toml` に clap 依存関係を追加
  - `clap = { version = "4", features = ["derive", "env"] }`
- [ ] T002 `router/src/cli/mod.rs` にCLIモジュールの基本構造を作成
  - Cliアーキタイプ構造体（clap derive）
  - Commands enum（None/User）
- [ ] T003 `router/src/cli/user.rs` にuserサブコマンドモジュールを作成
  - UserCommand enum（List/Add/Delete）
- [ ] T004 `router/src/lib.rs` に `pub mod cli;` を追加

## Phase 3.2: テストファースト (TDD)

**重要: これらのテストは記述され、実装前に失敗する必要がある**

- [ ] T005 [P] `router/tests/cli_help_test.rs` に --help 表示テスト
  - `llm-router --help` の出力に "user" サブコマンドが含まれることを確認
- [ ] T006 [P] `router/tests/cli_version_test.rs` に --version 表示テスト
  - `llm-router --version` の出力にバージョン番号が含まれることを確認
- [ ] T007 [P] `router/tests/cli_user_list_test.rs` に user list テスト
  - `llm-router user list` の出力形式を確認
- [ ] T008 [P] `router/tests/cli_user_add_test.rs` に user add テスト
  - 成功ケース: 新規ユーザー作成
  - 失敗ケース: パスワード8文字未満
  - 失敗ケース: 既存ユーザー名
- [ ] T009 [P] `router/tests/cli_user_delete_test.rs` に user delete テスト
  - 成功ケース: 既存ユーザー削除
  - 失敗ケース: 存在しないユーザー
- [ ] T010 [P] `router/tests/env_vars_test.rs` に環境変数テスト
  - `LLM_ROUTER_PORT` が使用されることを確認
  - 旧 `ROUTER_PORT` 使用時に警告が出ることを確認

## Phase 3.3: コア実装 (テストが失敗した後のみ)

- [ ] T011 `router/src/main.rs` を修正してCLI引数をパース
  - clap でコマンドライン引数を処理
  - 引数なし → サーバー起動（既存動作維持）
  - `--help` → ヘルプ表示
  - `--version` → バージョン表示
  - `user` → サブコマンドへディスパッチ
- [ ] T012 `router/src/cli/mod.rs` にCli構造体を実装
  - `#[command(version, about)]` 属性
  - 環境変数一覧をhelpに表示
- [ ] T013 `router/src/cli/user.rs` に user list を実装
  - DBからユーザー一覧を取得
  - ユーザー名とロールを表示
- [ ] T014 `router/src/cli/user.rs` に user add を実装
  - パスワードバリデーション（8文字以上）
  - bcryptでハッシュ化
  - DBに挿入
- [ ] T015 `router/src/cli/user.rs` に user delete を実装
  - ユーザー存在確認
  - DBから削除

## Phase 3.4: 環境変数統一

- [ ] T016 `router/src/config.rs` を新規作成（環境変数ヘルパー）
  - `get_env_with_fallback(new_name, old_name)` 関数
  - 旧変数使用時の警告ログ
- [ ] T017 `router/src/main.rs` の環境変数を更新
  - `ROUTER_HOST` → `LLM_ROUTER_HOST`（フォールバック付き）
  - `ROUTER_PORT` → `LLM_ROUTER_PORT`（フォールバック付き）
  - `JWT_SECRET` → `LLM_ROUTER_JWT_SECRET`（フォールバック付き）
  - `DATABASE_URL` → `LLM_ROUTER_DATABASE_URL`（フォールバック付き）
  - `HEALTH_CHECK_INTERVAL` → `LLM_ROUTER_HEALTH_CHECK_INTERVAL`
  - `NODE_TIMEOUT` → `LLM_ROUTER_NODE_TIMEOUT`
  - `LOAD_BALANCER_MODE` → `LLM_ROUTER_LOAD_BALANCER_MODE`
- [ ] T018 `router/src/auth/bootstrap.rs` の環境変数を更新
  - `ADMIN_USERNAME` → `LLM_ROUTER_ADMIN_USERNAME`（フォールバック付き）
  - `ADMIN_PASSWORD` → `LLM_ROUTER_ADMIN_PASSWORD`（フォールバック付き）
- [ ] T019 `router/src/logging.rs` の環境変数を更新
  - `ROUTER_LOG_LEVEL` を削除（レガシー）
  - `LLM_LOG_LEVEL` → `LLM_ROUTER_LOG_LEVEL`（フォールバック付き）
- [ ] T020 Cloud API環境変数を確認・更新
  - `OPENAI_API_KEY` → `LLM_ROUTER_OPENAI_API_KEY`
  - `ANTHROPIC_API_KEY` → `LLM_ROUTER_ANTHROPIC_API_KEY`
  - `GOOGLE_API_KEY` → `LLM_ROUTER_GOOGLE_API_KEY`

## Phase 3.5: JWT_SECRET ファイル永続化

- [ ] T021 [P] `router/tests/jwt_secret_test.rs` にJWT_SECRETファイル永続化テスト
  - ファイルが存在しない場合、UUIDv4で自動生成
  - ファイルが存在する場合、既存の値を読み込み
  - 環境変数が設定されている場合、環境変数を優先
  - ファイルのパーミッションが600であることを確認
- [ ] T022 `router/src/jwt_secret.rs` を新規作成
  - `get_or_create_jwt_secret()` 関数
  - UUIDv4生成
  - ファイル読み書き（パーミッション600）
- [ ] T023 `router/src/main.rs` のJWT_SECRET読み込みを更新
  - 新しい `get_or_create_jwt_secret()` を使用

## Phase 3.6: Node（C++）CLI

- [ ] T024 [P] `node/tests/unit/cli_help_test.cpp` に --help テスト
  - `llm-node --help` の出力に環境変数の説明が含まれることを確認
- [ ] T025 [P] `node/tests/unit/cli_version_test.cpp` に --version テスト
  - `llm-node --version` の出力にバージョン番号が含まれることを確認
- [ ] T026 `node/include/utils/version.h` を新規作成
  - `LLM_NODE_VERSION` 定数
- [ ] T027 `node/src/main.cpp` にCLI引数処理を追加
  - `--help` / `-h` → ヘルプ表示して終了
  - `--version` / `-V` → バージョン表示して終了
  - 引数なし → 既存動作（ノード起動）

## Phase 3.7: Node 環境変数統一

- [ ] T028 [P] `node/tests/unit/env_vars_test.cpp` に環境変数テスト
  - `LLM_NODE_MODELS_DIR` が使用されることを確認
  - 旧 `LLM_MODELS_DIR` 使用時に警告が出ることを確認
- [ ] T029 `node/src/utils/config.cpp` の環境変数を更新
  - `LLM_MODELS_DIR` → `LLM_NODE_MODELS_DIR`（フォールバック付き）
  - `LLM_HEARTBEAT_SECS` → `LLM_NODE_HEARTBEAT_SECS`（フォールバック付き）
  - `LLM_ALLOW_NO_GPU` → `LLM_NODE_ALLOW_NO_GPU`（フォールバック付き）
  - `LLM_BIND_ADDRESS` → `LLM_NODE_BIND_ADDRESS`（フォールバック付き）
- [ ] T030 `node/src/utils/logger.cpp` の環境変数を更新
  - `LLM_LOG_DIR` → `LLM_NODE_LOG_DIR`（フォールバック付き）
  - `LLM_LOG_LEVEL` → `LLM_NODE_LOG_LEVEL`（フォールバック付き）
  - `LLM_LOG_RETENTION_DAYS` → `LLM_NODE_LOG_RETENTION_DAYS`（フォールバック付き）

## Phase 3.8: 仕上げ

- [ ] T031 [P] `docs/authentication.md` を更新
  - 新しい環境変数名を記載
  - CLIコマンドの使用例を追加
- [ ] T032 [P] `INSTALL.md` を更新
  - 新しい環境変数名を記載
- [x] T033 [P] `README.md` を更新
  - 環境変数セクションを更新済み
- [ ] T034 すべてのテストを実行して合格を確認
  - Router: `cargo test` / `cargo clippy -- -D warnings` / `cargo fmt --check`
  - Node: `cmake --build build --target test`
- [ ] T035 手動テスト実行
  - `llm-router --help` を確認
  - `llm-router --version` を確認
  - `llm-router user list` を確認
  - `llm-router user add testuser --password testpass123` を確認
  - `llm-router user delete testuser` を確認
  - `llm-node --help` を確認
  - `llm-node --version` を確認

## 依存関係

```text
# Router (Rust)
T001 → T002 → T003 → T004
T004 → T005-T010 (並列可能)
T005-T010 → T011-T015
T011 → T012 → T013, T014, T015
T013, T014, T015 → T016
T016 → T017, T018, T019, T020
T017-T020 → T021-T023

# Node (C++)
T024-T025 (並列可能) → T026 → T027
T028 → T029, T030

# 仕上げ
T023, T030 → T031-T033 (並列可能)
T031-T034 → T035
```

## 並列実行例

```bash
# Phase 3.2 のテストを並列起動:
# T005-T010 を同時に実行可能（異なるファイル）

# Phase 3.6 のNode CLIテストを並列起動:
# T024-T025 を同時に実行可能（異なるファイル）

# Phase 3.8 のドキュメント更新を並列起動:
# T031-T033 を同時に実行可能（異なるファイル）

# Router と Node は独立して並列実行可能
```

## 検証チェックリスト

- [x] すべてのユーザーストーリーに対応するテストがある
- [x] すべてのテストが実装より先にある
- [x] 並列タスクは本当に独立している
- [x] 各タスクは正確なファイルパスを指定
- [x] 同じファイルを変更する[P]タスクがない
- [x] Node CLI要件が含まれている
- [x] JWT_SECRET永続化要件が含まれている

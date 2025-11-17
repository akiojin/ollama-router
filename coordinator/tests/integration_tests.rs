//! Integration tests entrypoint for model management

#[path = "integration/auto_download_test.rs"]
mod auto_download_test;

#[path = "integration/manual_distribution_test.rs"]
mod manual_distribution_test;

#[path = "integration/model_info_test.rs"]
mod model_info_test;

#[path = "integration/migration_test.rs"]
mod migration_test;

#[path = "integration/auth_flow_test.rs"]
mod auth_flow_test;

#[path = "integration/api_key_flow_test.rs"]
mod api_key_flow_test;

#[path = "integration/middleware_test.rs"]
mod middleware_test;

#[path = "integration/auth_disabled_test.rs"]
mod auth_disabled_test;

#[path = "integration/agent_token_test.rs"]
mod agent_token_test;

// Tests are defined inside the modules; this harness ensures they are built
// and executed when running `cargo test`.

//! Unit tests entrypoint for model management

#[path = "unit/gpu_model_selector_test.rs"]
mod gpu_model_selector_test;

#[path = "unit/model_repository_test.rs"]
mod model_repository_test;

#[path = "unit/password_test.rs"]
mod password_test;

#[path = "unit/jwt_test.rs"]
mod jwt_test;

// Tests are defined inside the modules; this harness ensures they are built
// and executed when running `cargo test`.

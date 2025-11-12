//! GUI関連のモジュール（システムトレイなど）。

#[cfg(any(target_os = "windows", target_os = "macos"))]
pub mod tray;

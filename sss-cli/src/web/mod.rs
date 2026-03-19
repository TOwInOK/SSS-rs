//! # Web server and file watching
//!
//! This module provides HTTP server functionality and file system monitoring.
//!
//! ## Submodules
//!
//! - `file_watcher`: File system monitoring with notify
//! - `serve`: HTTP server with axum

pub mod file_watcher;
pub mod serve;

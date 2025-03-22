//! Module that organizes drawing and rendering utilities for the application
//!
//! This module provides:
//! - Layout management and positioning
//! - Theme and styling configuration
//! - Content generation and formatting utilities
//! - Output formatting and rendering

pub mod layout;
/// Just prelude, no more
pub mod prelude;

/// Local result with [std::error::Error] under Box with Send + Sync
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

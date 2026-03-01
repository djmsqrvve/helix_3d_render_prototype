//! Logging system with structured output
//!
//! # Usage
//!
//! Local logging (default):
//! ```bash
//! cargo run
//! RUST_LOG=info cargo run
//! RUST_LOG=debug cargo run
//! ```
//!
//! # Environment Variables
//!
//! - `RUST_LOG` - Set log level filter (e.g., `debug`, `info,bevy=warn`)
//!
//! # Remote Logging
//!
//! To implement remote logging, enable the `remote-log` feature and provide
//! a `--log-endpoint` URL. See docs/DEV_LOGGING.md for details.

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initialize the logging system
pub fn init_logging(_log_endpoint: Option<&str>) {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,bevy_3d_renderer=debug,wgpu=warn"));

    // Standard local logging
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_timer(tracing_subscriber::fmt::time::time());

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();

    tracing::info!("Logging initialized (local mode)");
    tracing::debug!("Set RUST_LOG=debug for verbose output");

    if _log_endpoint.is_some() {
        tracing::warn!("--log-endpoint provided but remote logging not yet implemented");
    }
}

/// Log structured event for analysis
#[macro_export]
macro_rules! log_event {
    ($event:expr, $($key:ident = $value:expr),*) => {
        tracing::info!(event = $event, $($key = $value),*)
    };
}

/// Log metric for monitoring
#[macro_export]
macro_rules! log_metric {
    ($name:expr, $value:expr) => {
        tracing::info!(metric_name = $name, metric_value = $value)
    };
}

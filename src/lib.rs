pub mod api;
pub mod network;
pub mod scanner;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Information about a host running the LCN service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostInfo {
    pub hostname: String,
    pub hostipv4: String,
}

/// LCN service port
pub const LCN_PORT: u16 = 7979;

/// Connection timeout for scanning (in seconds)
pub const SCAN_TIMEOUT_SECS: u64 = 2;

/// Maximum concurrent connections during scan
pub const MAX_CONCURRENT_SCANS: usize = 64;

#[derive(Error, Debug)]
pub enum LcnError {
    #[error("Failed to get local IP address: {0}")]
    LocalIpError(String),

    #[error("Network scan failed: {0}")]
    ScanError(String),

    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

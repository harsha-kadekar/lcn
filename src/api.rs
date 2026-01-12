use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::network::{generate_subnet_ips, get_hostname, get_local_ip};
use crate::scanner::scan_subnet;
use crate::HostInfo;

/// Handler for GET /hostinfo
/// Returns the hostname and IPv4 address of the current host
pub async fn hostinfo() -> Result<impl IntoResponse, (StatusCode, String)> {
    let hostname = get_hostname().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get hostname: {}", e),
        )
    })?;

    let ip = get_local_ip().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get local IP: {}", e),
        )
    })?;

    let info = HostInfo {
        hostname,
        hostipv4: ip.to_string(),
    };

    Ok(Json(info))
}

/// Handler for GET /scanhosts
/// Scans the local network for other hosts running LCN service
pub async fn scanhosts() -> Result<impl IntoResponse, (StatusCode, String)> {
    let local_ip = get_local_ip().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get local IP: {}", e),
        )
    })?;

    let subnet_ips = generate_subnet_ips(local_ip);

    let hosts = scan_subnet(subnet_ips).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Scan failed: {}", e),
        )
    })?;

    Ok(Json(hosts))
}

use crate::{HostInfo, LcnError, LCN_PORT, MAX_CONCURRENT_SCANS, SCAN_TIMEOUT_SECS};
use std::net::Ipv4Addr;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::Semaphore;
use tokio::time::timeout;

/// Check if a host has port 7979 open (potential LCN service)
async fn is_port_open(ip: Ipv4Addr) -> bool {
    let addr = format!("{}:{}", ip, LCN_PORT);
    let connect_timeout = Duration::from_secs(SCAN_TIMEOUT_SECS);

    timeout(connect_timeout, TcpStream::connect(&addr))
        .await
        .is_ok_and(|result| result.is_ok())
}

/// Fetch HostInfo from a remote LCN service
async fn fetch_host_info(ip: Ipv4Addr) -> Option<HostInfo> {
    let url = format!("http://{}:{}/hostinfo", ip, LCN_PORT);
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(SCAN_TIMEOUT_SECS))
        .build()
        .ok()?;

    client
        .get(&url)
        .send()
        .await
        .ok()?
        .json::<HostInfo>()
        .await
        .ok()
}

/// Scan a single IP address for LCN service
async fn scan_single_host(ip: Ipv4Addr, semaphore: &Semaphore) -> Option<HostInfo> {
    let _permit = semaphore.acquire().await.ok()?;

    if is_port_open(ip).await {
        fetch_host_info(ip).await
    } else {
        None
    }
}

/// Scan all IPs in the subnet for LCN services
pub async fn scan_subnet(ips: Vec<Ipv4Addr>) -> Result<Vec<HostInfo>, LcnError> {
    let semaphore = Semaphore::new(MAX_CONCURRENT_SCANS);
    let semaphore = std::sync::Arc::new(semaphore);

    let mut handles = Vec::with_capacity(ips.len());

    for ip in ips {
        let sem = semaphore.clone();
        let handle = tokio::spawn(async move { scan_single_host(ip, &sem).await });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        if let Ok(Some(host_info)) = handle.await {
            results.push(host_info);
        }
    }

    // Sort by IP address for consistent output
    results.sort_by(|a, b| a.hostipv4.cmp(&b.hostipv4));

    Ok(results)
}

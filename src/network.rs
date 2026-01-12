use crate::LcnError;
use std::net::Ipv4Addr;

/// Get the local IPv4 address of this machine
pub fn get_local_ip() -> Result<Ipv4Addr, LcnError> {
    local_ip_address::local_ip()
        .map_err(|e| LcnError::LocalIpError(e.to_string()))
        .and_then(|ip| match ip {
            std::net::IpAddr::V4(ipv4) => Ok(ipv4),
            std::net::IpAddr::V6(_) => Err(LcnError::LocalIpError(
                "Expected IPv4 address, got IPv6".to_string(),
            )),
        })
}

/// Get the hostname of this machine
pub fn get_hostname() -> Result<String, LcnError> {
    hostname::get()
        .map_err(|e| LcnError::IoError(e))?
        .into_string()
        .map_err(|_| LcnError::LocalIpError("Invalid hostname encoding".to_string()))
}

/// Generate all IPv4 addresses in the /24 subnet of the given IP
/// For example, if given 192.168.1.10, generates 192.168.1.1 through 192.168.1.254
pub fn generate_subnet_ips(local_ip: Ipv4Addr) -> Vec<Ipv4Addr> {
    let octets = local_ip.octets();
    (1..=254)
        .map(|last_octet| Ipv4Addr::new(octets[0], octets[1], octets[2], last_octet))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_subnet_ips() {
        let ip = Ipv4Addr::new(192, 168, 1, 100);
        let ips = generate_subnet_ips(ip);

        assert_eq!(ips.len(), 254);
        assert_eq!(ips[0], Ipv4Addr::new(192, 168, 1, 1));
        assert_eq!(ips[253], Ipv4Addr::new(192, 168, 1, 254));
    }
}

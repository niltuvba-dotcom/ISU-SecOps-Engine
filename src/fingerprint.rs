use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;
use serde::{Serialize, Deserialize};
use regex::Regex;
use futures::stream::{self, StreamExt};
use tokio::sync::mpsc;

/// Represents a successful service identification on a given target and port.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FingerprintResult {
    /// IP address or hostname of the scanned target.
    pub target: String,
    /// TCP port number.
    pub port: u16,
    /// Connection state (usually "open").
    pub state: String,
    /// Identified service name (e.g., SSH, HTTP).
    pub service: String,
    /// Detected version string or "unknown".
    pub version: String,
}

/// Expands a single target string (IP, hostname, or CIDR) into a list of individual IP addresses.
pub fn expand_target(target: &str) -> Vec<String> {
    if let Ok(net) = target.parse::<ipnet::IpNet>() {
        net.hosts().map(|h| h.to_string()).collect()
    } else {
        vec![target.to_string()]
    }
}

/// Orchestrates an asynchronous scan for multiple targets and ports with controlled concurrency.
/// 
/// Results are streamed through an unbounded mpsc channel for real-time reporting.
pub async fn run_fingerprint_streaming(
    target: &str,
    ports: Vec<u16>,
    concurrency: usize,
    timeout_sec: u64,
    tx: mpsc::UnboundedSender<FingerprintResult>,
) -> anyhow::Result<()> {
    let targets = expand_target(target);
    let mut tasks = vec![];

    for t in targets {
        for &p in &ports {
            tasks.push((t.clone(), p));
        }
    }

    let stream = stream::iter(tasks).map(|(t, p)| {
        let tx = tx.clone();
        async move {
            if let Ok(Some(res)) = fingerprint_port(&t, p, timeout_sec).await {
                let _ = tx.send(res);
            }
        }
    }).buffer_unordered(concurrency);

    stream.collect::<Vec<_>>().await;
    Ok(())
}

/// Attempts to connect to a specific port and identify its service through banner grabbing.
pub async fn fingerprint_port(target: &str, port: u16, timeout_sec: u64) -> anyhow::Result<Option<FingerprintResult>> {
    let addr = format!("{}:{}", target, port);
    let socket_addr: SocketAddr = addr.parse().or_else(|_| {
        // Simple hostname resolution attempt for basic cases
        std::net::ToSocketAddrs::to_socket_addrs(&addr)
            .map(|mut iter| iter.next().unwrap())
    })?;

    let connect_timeout = Duration::from_secs(timeout_sec);
    
    // Phase 1: Connection
    let mut stream = match timeout(connect_timeout, TcpStream::connect(&socket_addr)).await {
        Ok(Ok(s)) => s,
        _ => return Ok(None),
    };

    // Phase 2: Banner Grabbing
    let mut buffer = [0u16; 1]; // Dummy to trigger some responses if needed
    let _ = stream.write_all(b"\r\n").await; // Gentle nudge for some services
    
    let mut banner_buffer = [0u8; 1024];
    let read_timeout = Duration::from_secs(2);
    
    let banner = match timeout(read_timeout, stream.read(&mut banner_buffer)).await {
        Ok(Ok(n)) if n > 0 => {
            String::from_utf8_lossy(&banner_buffer[..n]).to_string()
        }
        _ => {
            // Attempt active probe if no immediate banner
            let mut probe_buffer = [0u8; 1024];
            let _ = stream.write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n").await;
            match timeout(read_timeout, stream.read(&mut probe_buffer)).await {
                Ok(Ok(n)) if n > 0 => String::from_utf8_lossy(&probe_buffer[..n]).to_string(),
                _ => "unknown".to_string(),
            }
        }
    };

    let (service, version) = if banner == "unknown" {
        ("unknown".to_string(), "unknown".to_string())
    } else {
        identify_service(&banner).unwrap_or(("unknown".to_string(), "unknown".to_string()))
    };

    Ok(Some(FingerprintResult {
        target: target.to_string(),
        port,
        state: "open".to_string(),
        service,
        version,
    }))
}

/// Identifies common network services and extracts versioning information using regex patterns.
pub fn identify_service(banner: &str) -> Option<(String, String)> {
    let patterns = [
        ("SSH", r"SSH-(\d+\.\d+)-"),
        ("HTTP", r"HTTP/\d+\.\d+\s+(\d+)"),
        ("Redis", r"redis_version:(\d+\.\d+\.\d+)"),
        ("PostgreSQL", r"PostgreSQL"),
        ("SMTP", r"220.*SMTP"),
        ("FTP", r"220.*FTP"),
    ];

    for (name, pattern) in patterns {
        let re = Regex::new(pattern).ok()?;
        if let Some(caps) = re.captures(banner) {
            let version = caps.get(1).map(|m| m.as_str()).unwrap_or("unknown").to_string();
            return Some((name.to_string(), version));
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_target_single_ip() {
        let targets = expand_target("127.0.0.1");
        assert_eq!(targets.len(), 1);
        assert_eq!(targets[0], "127.0.0.1");
    }

    #[test]
    fn test_expand_target_cidr() {
        let targets = expand_target("192.168.1.0/30");
        // /30 has 2 usable hosts (.1, .2) in some implementations, 
        // but ipnet hosts() covers all assignable IPs.
        assert_eq!(targets.len(), 2); 
    }

    #[test]
    fn test_identify_service_ssh() {
        let banner = "SSH-2.0-OpenSSH_8.2p1 Ubuntu-4ubuntu0.5";
        let res = identify_service(banner);
        assert!(res.is_some());
        let (name, ver) = res.unwrap();
        assert_eq!(name, "SSH");
        assert_eq!(ver, "2.0");
    }

    #[test]
    fn test_identify_service_http() {
        let banner = "HTTP/1.1 200 OK\r\nServer: nginx";
        let res = identify_service(banner);
        assert!(res.is_some());
        let (name, ver) = res.unwrap();
        assert_eq!(name, "HTTP");
        assert_eq!(ver, "200");
    }
}

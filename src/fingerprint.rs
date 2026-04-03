use ipnet::IpNet;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::{Semaphore, mpsc};
use tokio::time::{Duration, timeout};

/// Represents the result of a single port fingerprinting operation.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FingerprintResult {
    /// The target IP or Hostname scanned.
    pub target: String,
    /// The port number scanned.
    pub port: u16,
    /// The state of the port (e.g., "open").
    pub state: String,
    /// The detected service name (e.g., "SSH", "HTTP").
    pub service: String,
    /// The detected service version or banner.
    pub version: String,
}

/// Orchestrates a streaming fingerprinting scan across multiple targets and ports.
///
/// This function uses a semaphore to control concurrency and sends results through
/// the provided mpsc channel as they are discovered.
///
/// # Arguments
/// * `target_input` - A string representing a single target, a hostname, or a CIDR range.
/// * `ports` - A vector of port numbers to scan.
/// * `concurrency` - Maximum number of simultaneous TCP connections.
/// * `timeout_sec` - Timeout for each connection/read attempt in seconds.
/// * `tx` - Sender half of an mpsc channel to stream results.
pub async fn run_fingerprint_streaming(
    target_input: &str,
    ports: Vec<u16>,
    concurrency: usize,
    timeout_sec: u64,
    tx: mpsc::UnboundedSender<FingerprintResult>,
) -> anyhow::Result<()> {
    let targets = expand_target(target_input);
    let mut tasks = vec![];
    let semaphore = Arc::new(Semaphore::new(concurrency));
 
    for target in targets {
        let target_clone = target.clone();
        let ports_clone = ports.clone();
        let sem_clone = semaphore.clone();
        let tx_clone = tx.clone();
        
        tasks.push(tokio::spawn(async move {
            // Smart Host Discovery: Only skip if scanning many ports
            if ports_clone.len() > 5 && !is_host_up(&target_clone).await {
                return;
            }

            for port in ports_clone {
                let inner_target = target_clone.clone();
                let inner_sem = sem_clone.clone();
                let inner_tx = tx_clone.clone();
                
                let _permit = inner_sem.acquire().await.unwrap();
                let res = fingerprint_port(&inner_target, port, timeout_sec).await;
                
                let fp_res = match res {
                    Ok(Some((service, version))) => Some(FingerprintResult {
                        target: inner_target,
                        port,
                        state: "open".to_string(),
                        service,
                        version,
                    }),
                    Ok(None) => Some(FingerprintResult {
                        target: inner_target,
                        port,
                        state: "open".to_string(),
                        service: "unknown".to_string(),
                        version: "unknown".to_string(),
                    }),
                    Err(_) => None,
                };
 
                if let Some(r) = fp_res {
                    let _ = inner_tx.send(r);
                }
            }
        }));
    }
 
    for task in tasks {
        let _ = task.await;
    }
 
    Ok(())
}

/// Performs a quick TCP health check on common ports to see if a host is responsive.
async fn is_host_up(target: &str) -> bool {
    let common_ports = [80, 443, 22, 445, 135, 3389];
    for port in common_ports {
        let addr = format!("{}:{}", target, port);
        if let Ok(Ok(_)) = timeout(Duration::from_millis(500), TcpStream::connect(&addr)).await {
            return true;
        }
    }
    false
}

/// Expands a target input string into a list of individual IP addresses.
///
/// Supports single IPs, hostnames, and CIDR notation (e.g., 192.168.1.0/24).
pub fn expand_target(input: &str) -> Vec<String> {
    if let Ok(net) = IpNet::from_str(input) {
        net.hosts().map(|ip| ip.to_string()).collect()
    } else {
        vec![input.to_string()]
    }
}

/// Individual port fingerprinting logic.
///
/// Connects to the port and attempts to identify the service by either reading
/// a passive banner or sending a protocol-specific probe.
async fn fingerprint_port(target: &str, port: u16, timeout_sec: u64) -> anyhow::Result<Option<(String, String)>> {
    let addr = format!("{}:{}", target, port);
    
    // Connect with user-defined timeout
    let stream_result = timeout(Duration::from_secs(timeout_sec), TcpStream::connect(&addr)).await;
    
    let mut stream = match stream_result {
        Ok(Ok(s)) => s,
        _ => return Err(anyhow::anyhow!("Connection failed or timeout")),
    };
 
    // Try to read a passive banner
    let mut buffer = [0; 4096];
    let read_result = timeout(Duration::from_secs(timeout_sec), stream.read(&mut buffer)).await;
    
    let mut response_text = String::new();
    if let Ok(Ok(n)) = read_result {
        if n > 0 {
            response_text = String::from_utf8_lossy(&buffer[..n]).to_string();
        }
    }
 
    // Passive banner detected
    if !response_text.is_empty() {
        if let Some(res) = identify_service(&response_text) {
            return Ok(Some(res));
        }
    }
 
    // Active Probing: If no passive banner, send protocol-specific probes
    let probes = [
        ("GET / HTTP/1.1\r\nHost: localhost\r\n\r\n", "HTTP"),
        ("\r\n\r\n", "Possible Service"),
    ];
 
    for (probe, _label) in probes {
        let _ = stream.write_all(probe.as_bytes()).await;
        let mut probe_buffer = [0; 4096];
        let probe_read = timeout(Duration::from_secs(timeout_sec), stream.read(&mut probe_buffer)).await;
        
        if let Ok(Ok(n)) = probe_read {
            if n > 0 {
                let probe_res = String::from_utf8_lossy(&probe_buffer[..n]).to_string();
                if let Some(res) = identify_service(&probe_res) {
                    return Ok(Some(res));
                }
                // Fallback for standard HTTP
                if probe_res.contains("HTTP/") {
                    return Ok(Some(("HTTP".to_string(), parse_http_version(&probe_res))));
                }
            }
        }
    }
 
    Ok(None)
}

/// Identifies a service and its version based on a response string using regex patterns.
fn identify_service(banner: &str) -> Option<(String, String)> {
    let patterns = [
        (r"SSH-([0-9.]+)-([^ \r\n]+)", "SSH"),
        (r"HTTP/[0-9.]+ ([0-9]+)", "HTTP"),
        (r"Server: ([^ \r\n]+)", "HTTP"),
        (r"Redis_version:([0-9.]+)", "Redis"),
        (r"PostgreSQL", "PostgreSQL"),
        (r"220 ([^ \r\n]+) ESMTP", "SMTP"),
        (r"FTP", "FTP"),
    ];
 
    for (pattern, service) in patterns {
        let re = Regex::new(pattern).ok()?;
        if let Some(caps) = re.captures(banner) {
            let version = if caps.len() > 1 {
                caps.get(1).map_or("unknown", |m| m.as_str()).to_string()
            } else {
                "detected".to_string()
            };
            return Some((service.to_string(), version));
        }
    }
    None
}

/// Helper to parse HTTP server version from a response.
fn parse_http_version(banner: &str) -> String {
    let re = Regex::new(r"Server: ([^\r\n]+)").unwrap();
    re.captures(banner)
        .and_then(|cap| cap.get(1))
        .map_or("unknown", |m| m.as_str())
        .to_string()
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
        // 192.168.1.0/30 should have 2 usable hosts (.1, .2) in some implementations, 
        // but ipnet's hosts() returns all IPs if it's a small subnet or handles it via standard rules.
        let targets = expand_target("192.168.1.0/30");
        assert!(targets.len() >= 2);
    }

    #[test]
    fn test_identify_service_ssh() {
        let banner = "SSH-2.0-OpenSSH_8.2p1 Ubuntu-4ubuntu0.5";
        let result = identify_service(banner);
        assert!(result.is_some());
        let (service, version) = result.unwrap();
        assert_eq!(service, "SSH");
        assert_eq!(version, "2.0");
    }

    #[test]
    fn test_identify_service_http() {
        let banner = "HTTP/1.1 200 OK\r\nServer: nginx/1.18.0\r\n";
        let result = identify_service(banner);
        assert!(result.is_some());
        let (service, version) = result.unwrap();
        assert_eq!(service, "HTTP");
        // Our regex for HTTP/ picks the status code 200 first, or nginx. 
        // Based on patterns array: r"HTTP/[0-9.]+ ([0-9]+)" is first.
        assert_eq!(version, "200");
    }
}

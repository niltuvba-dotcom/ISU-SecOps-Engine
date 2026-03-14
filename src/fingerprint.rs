use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{timeout, Duration};
use regex::Regex;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct FingerprintResult {
    pub port: u16,
    pub state: String,
    pub service: String,
    pub version: String,
}

pub async fn run_fingerprint(target: &str, ports: Vec<u16>) -> anyhow::Result<()> {
    let results = run_fingerprint_logic(target, ports).await?;

    println!("{:<10} {:<15} {:<30}", "PORT", "STATE", "SERVICE/VERSION");
    println!("{}", "-".repeat(55));

    for res in results {
        println!(
            "{:<10} {:<15} {}/{}",
            format!("{}/tcp", res.port),
            res.state,
            res.service,
            res.version
        );
    }
    Ok(())
}

pub async fn run_fingerprint_logic(target: &str, ports: Vec<u16>) -> anyhow::Result<Vec<FingerprintResult>> {
    let mut tasks = vec![];

    for port in ports.clone() {
        let target_clone = target.to_string();
        tasks.push(tokio::spawn(async move {
            let res = fingerprint_port(&target_clone, port).await;
            (port, res)
        }));
    }

    let mut results = vec![];

    for task in tasks {
        let (port, res) = task.await.unwrap();
        match res {
            Ok(Some((service, version))) => {
                results.push(FingerprintResult {
                    port,
                    state: "open".to_string(),
                    service,
                    version,
                });
            }
            Ok(None) => {
                results.push(FingerprintResult {
                    port,
                    state: "open".to_string(),
                    service: "unknown".to_string(),
                    version: "unknown".to_string(),
                });
            }
            Err(_) => {
                // optionally handle closed ports
            }
        }
    }

    // Sort by port number
    results.sort_by_key(|r| r.port);

    Ok(results)
}
async fn fingerprint_port(target: &str, port: u16) -> anyhow::Result<Option<(String, String)>> {
    let addr = format!("{}:{}", target, port);
    
    // Connect with a 3-second timeout
    let stream_result = timeout(Duration::from_secs(3), TcpStream::connect(&addr)).await;
    
    let mut stream = match stream_result {
        Ok(Ok(s)) => s,
        _ => return Err(anyhow::anyhow!("Connection failed or timeout")),
    };

    // Try to read a passive banner (e.g. SSH, FTP send banner upon connection)
    let mut buffer = [0; 4096];
    let read_result = timeout(Duration::from_secs(3), stream.read(&mut buffer)).await;
    
    let mut response_text = String::new();
    
    if let Ok(Ok(n)) = read_result {
        if n > 0 {
            response_text = String::from_utf8_lossy(&buffer[..n]).to_string();
        }
    }

    // If no passive banner, send an active probe (e.g. HTTP requires a request first)
    if response_text.is_empty() || response_text.trim().is_empty() {
        let http_probe = b"GET / HTTP/1.0\r\n\r\n";
        let _ = stream.write_all(http_probe).await;
        
        let read_result = timeout(Duration::from_secs(3), stream.read(&mut buffer)).await;
        if let Ok(Ok(n)) = read_result {
            if n > 0 {
                response_text = String::from_utf8_lossy(&buffer[..n]).to_string();
            }
        }
    }
    
    if response_text.is_empty() {
        return Ok(None);
    }

    // Match signature against collected banner
    Ok(match_signature(&response_text))
}

fn match_signature(banner: &str) -> Option<(String, String)> {
    // 1. SSH Signature
    if let Ok(re) = Regex::new(r"(?i)^SSH-([\d.]+)-([^\r\n]+)") {
        if let Some(caps) = re.captures(banner) {
            return Some(("SSH".to_string(), caps[2].to_string()));
        }
    }

    // 2. HTTP Signature
    if banner.contains("HTTP/1.") || banner.contains("HTTP/2") || banner.contains("HTTP/0.") {
        let mut version = "unknown".to_string();
        if let Ok(re) = Regex::new(r"(?i)Server:\s*([^\r\n]+)") {
            if let Some(caps) = re.captures(banner) {
                version = caps[1].to_string();
            }
        }
        return Some(("HTTP".to_string(), version));
    }

    // 3. FTP Signature
    if let Ok(re) = Regex::new(r"(?i)^220[- ]([^\r\n]+)") {
        if let Some(caps) = re.captures(banner) {
            return Some(("FTP".to_string(), caps[1].to_string()));
        }
    }
    
    // 4. MySQL Signature Heuristic
    // MySQL's greeting packet is binary but contains human-readable version like "5.5.5-10.4.24-MariaDB"
    if banner.contains("MariaDB") || banner.contains("mysql_native_password") || banner.contains("caching_sha2_password") {
        if let Ok(re) = Regex::new(r"([\d]+\.[\d]+\.[\d]+(-MariaDB)?)") {
            if let Some(caps) = re.captures(banner) {
                return Some(("MySQL".to_string(), caps[1].to_string()));
            }
        }
        return Some(("MySQL".to_string(), "unknown".to_string()));
    }

    // 5. Fallback - show truncated banner if it's text
    let clean_banner = banner.lines().next().unwrap_or("").trim().to_string();
    if !clean_banner.is_empty() && clean_banner.chars().all(|c| c.is_ascii_graphic() || c.is_ascii_whitespace()) {
        let display_banner = if clean_banner.len() > 30 {
            format!("{}...", &clean_banner[..30])
        } else {
            clean_banner
        };
        return Some(("unknown".to_string(), display_banner));
    }

    None
}

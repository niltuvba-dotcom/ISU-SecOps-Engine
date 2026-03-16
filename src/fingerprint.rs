use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{timeout, Duration};
use tokio::sync::Semaphore;
use std::sync::Arc;
use regex::Regex;
use serde::Serialize;
use ipnet::IpNet;
use std::str::FromStr;

#[derive(Serialize, Clone, Debug)]
pub struct FingerprintResult {
    pub target: String,
    pub port: u16,
    pub state: String,
    pub service: String,
    pub version: String,
}

pub async fn run_fingerprint(target: &str, ports: Vec<u16>, concurrency: usize, timeout_sec: u64) -> anyhow::Result<()> {
    let results = run_fingerprint_logic(target, ports, concurrency, timeout_sec).await?;

    println!("{:<20} {:<10} {:<15} {:<30}", "TARGET", "PORT", "STATE", "SERVICE/VERSION");
    println!("{}", "-".repeat(75));
 
    for res in results {
        println!(
            "{:<20} {:<10} {:<15} {}/{}",
            res.target,
            format!("{}/tcp", res.port),
            res.state,
            res.service,
            res.version
        );
    }
    Ok(())
}

pub async fn run_fingerprint_logic(
    target_input: &str,
    ports: Vec<u16>,
    concurrency: usize,
    timeout_sec: u64,
) -> anyhow::Result<Vec<FingerprintResult>> {
    let targets = expand_target(target_input);
    let mut tasks = vec![];
    let semaphore = Arc::new(Semaphore::new(concurrency));
 
    for target in targets {
        for port in ports.clone() {
            let target_clone = target.clone();
            let sem_clone = semaphore.clone();
            tasks.push(tokio::spawn(async move {
                let _permit = sem_clone.acquire().await.unwrap();
                let res = fingerprint_port(&target_clone, port, timeout_sec).await;
                (target_clone, port, res)
            }));
        }
    }
 
    let mut results = vec![];
 
    for task in tasks {
        let (target, port, res) = task.await.unwrap();
        match res {
            Ok(Some((service, version))) => {
                results.push(FingerprintResult {
                    target,
                    port,
                    state: "open".to_string(),
                    service,
                    version,
                });
            }
            Ok(None) => {
                results.push(FingerprintResult {
                    target,
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
 
    // Sort by target then port number
    results.sort_by(|a, b| {
        a.target.cmp(&b.target).then(a.port.cmp(&b.port))
    });
 
    Ok(results)
}

fn expand_target(input: &str) -> Vec<String> {
    if let Ok(net) = IpNet::from_str(input) {
        net.hosts().map(|ip| ip.to_string()).collect()
    } else {
        vec![input.to_string()]
    }
}

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

    // If no passive banner, send an active probe (e.g. HTTP requires a request first)
    if response_text.is_empty() || response_text.trim().is_empty() {
        let probes = [
            ("HTTP", b"GET / HTTP/1.0\r\n\r\n".as_slice()),
            ("Redis", b"PING\r\n".as_slice()),
            ("Redis_Info", b"INFO\r\n".as_slice()),
        ];

        for (_name, probe) in probes {
            let _ = stream.write_all(probe).await;
            
            let read_result = timeout(Duration::from_secs(2), stream.read(&mut buffer)).await;
            if let Ok(Ok(n)) = read_result {
                if n > 0 {
                    let text = String::from_utf8_lossy(&buffer[..n]).to_string();
                    if !text.trim().is_empty() {
                        response_text.push_str(&text);
                        break;
                    }
                }
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
        if banner.to_uppercase().contains("FTP") || banner.len() > 10 {
             if let Some(caps) = re.captures(banner) {
                return Some(("FTP".to_string(), caps[1].to_string()));
            }
        }
    }

    // 4. SMTP Signature
    if let Ok(re) = Regex::new(r"(?i)^220\s+([^\r\n ]+)\s+ESMTP") {
        if let Some(caps) = re.captures(banner) {
            return Some(("SMTP".to_string(), caps[1].to_string()));
        }
    }

    // 5. POP3 Signature
    if banner.starts_with("+OK") {
        let version = banner.split_whitespace().nth(1).unwrap_or("unknown").to_string();
        return Some(("POP3".to_string(), version));
    }

    // 6. IMAP Signature
    if banner.contains("* OK") && banner.to_uppercase().contains("IMAP") {
        return Some(("IMAP".to_string(), "unknown".to_string()));
    }

    // 7. Redis Signature
    if banner.starts_with("+PONG") || banner.contains("redis_version") {
        let mut version = "unknown".to_string();
        if let Ok(re) = Regex::new(r"redis_version:([^\r\n]+)") {
            if let Some(caps) = re.captures(banner) {
                version = caps[1].to_string();
            }
        }
        return Some(("Redis".to_string(), version));
    }

    // 8. Postgres (Heuristic)
    if banner.contains("PostgreSQL") || (banner.len() >= 5 && banner.as_bytes()[0] == b'R') {
        return Some(("Postgres".to_string(), "unknown".to_string()));
    }
    
    // 9. MySQL Signature Heuristic
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

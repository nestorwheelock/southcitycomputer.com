// South City Computer - Client Performance & Network Diagnostics Tool
// Measures load times, runs network diagnostics, and visualizes routes

use std::io::{Read, Write, BufRead, BufReader};
use std::net::{TcpStream, ToSocketAddrs, IpAddr};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use std::collections::HashMap;

const DEFAULT_TARGET: &str = "southcitycomputer.com";

#[derive(Debug, Clone)]
struct PerformanceMetrics {
    dns_lookup_ms: f64,
    tcp_connect_ms: f64,
    tls_handshake_ms: f64,
    ttfb_ms: f64,  // Time to First Byte
    download_ms: f64,
    total_ms: f64,
    response_size: usize,
    status_code: u16,
}

#[derive(Debug, Clone)]
struct TraceHop {
    hop_number: u8,
    ip_address: Option<String>,
    hostname: Option<String>,
    rtt_ms: Vec<f64>,
    is_target: bool,
}

#[derive(Debug)]
struct NetworkDiagnostics {
    target: String,
    resolved_ip: Option<String>,
    hops: Vec<TraceHop>,
    total_hops: u8,
    packet_loss: f64,
}

fn resolve_hostname(hostname: &str) -> Result<(String, f64), String> {
    let start = Instant::now();

    let addr = format!("{}:80", hostname);
    match addr.to_socket_addrs() {
        Ok(mut addrs) => {
            if let Some(addr) = addrs.next() {
                let elapsed = start.elapsed().as_secs_f64() * 1000.0;
                Ok((addr.ip().to_string(), elapsed))
            } else {
                Err("No addresses found".to_string())
            }
        }
        Err(e) => Err(format!("DNS resolution failed: {}", e)),
    }
}

fn measure_tcp_connect(ip: &str, port: u16) -> Result<(TcpStream, f64), String> {
    let start = Instant::now();
    let addr = format!("{}:{}", ip, port);

    match TcpStream::connect_timeout(
        &addr.parse().map_err(|e| format!("Invalid address: {}", e))?,
        Duration::from_secs(10)
    ) {
        Ok(stream) => {
            let elapsed = start.elapsed().as_secs_f64() * 1000.0;
            Ok((stream, elapsed))
        }
        Err(e) => Err(format!("TCP connect failed: {}", e)),
    }
}

fn measure_http_request(mut stream: TcpStream, host: &str, path: &str) -> Result<(f64, f64, usize, u16), String> {
    stream.set_read_timeout(Some(Duration::from_secs(30)))
        .map_err(|e| format!("Set timeout failed: {}", e))?;

    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nUser-Agent: SCC-PerfClient/1.0\r\n\r\n",
        path, host
    );

    let send_start = Instant::now();
    stream.write_all(request.as_bytes())
        .map_err(|e| format!("Write failed: {}", e))?;

    // Read first byte (TTFB)
    let mut first_byte = [0u8; 1];
    stream.read_exact(&mut first_byte)
        .map_err(|e| format!("Read failed: {}", e))?;
    let ttfb = send_start.elapsed().as_secs_f64() * 1000.0;

    // Read rest of response
    let mut response = vec![first_byte[0]];
    stream.read_to_end(&mut response)
        .map_err(|e| format!("Read failed: {}", e))?;
    let download_time = send_start.elapsed().as_secs_f64() * 1000.0 - ttfb;

    // Parse status code
    let response_str = String::from_utf8_lossy(&response);
    let status_code = response_str
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|code| code.parse().ok())
        .unwrap_or(0);

    Ok((ttfb, download_time, response.len(), status_code))
}

fn measure_endpoint(host: &str, path: &str, port: u16) -> Result<PerformanceMetrics, String> {
    // DNS lookup
    let (ip, dns_ms) = resolve_hostname(host)?;

    // TCP connect
    let (stream, tcp_ms) = measure_tcp_connect(&ip, port)?;

    // HTTP request (includes TTFB and download)
    let (ttfb_ms, download_ms, size, status) = measure_http_request(stream, host, path)?;

    let total = dns_ms + tcp_ms + ttfb_ms + download_ms;

    Ok(PerformanceMetrics {
        dns_lookup_ms: dns_ms,
        tcp_connect_ms: tcp_ms,
        tls_handshake_ms: 0.0, // HTTP only for now
        ttfb_ms,
        download_ms,
        total_ms: total,
        response_size: size,
        status_code: status,
    })
}

fn run_traceroute(target: &str) -> Result<NetworkDiagnostics, String> {
    println!("Running traceroute to {}...", target);
    println!();

    // Resolve target first
    let resolved_ip = resolve_hostname(target).ok().map(|(ip, _)| ip);

    let output = Command::new("traceroute")
        .args(&["-n", "-q", "3", "-w", "2", target])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    let output = match output {
        Ok(o) => o,
        Err(_) => {
            // Try tracert on Windows
            Command::new("tracert")
                .args(&["-d", "-w", "2000", target])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .map_err(|e| format!("Traceroute failed: {}", e))?
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut hops = Vec::new();
    let mut hop_num = 0u8;

    for line in stdout.lines().skip(1) {
        hop_num += 1;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        let mut hop = TraceHop {
            hop_number: hop_num,
            ip_address: None,
            hostname: None,
            rtt_ms: Vec::new(),
            is_target: false,
        };

        for part in &parts {
            // Check if it's an IP address
            if part.parse::<IpAddr>().is_ok() {
                hop.ip_address = Some(part.to_string());
                if let Some(ref resolved) = resolved_ip {
                    if part == resolved {
                        hop.is_target = true;
                    }
                }
            }
            // Check if it's a timing (ends with "ms" or is a number)
            else if let Ok(ms) = part.trim_end_matches("ms").parse::<f64>() {
                hop.rtt_ms.push(ms);
            }
        }

        if hop.ip_address.is_some() || !hop.rtt_ms.is_empty() {
            hops.push(hop);
        }

        if hop_num >= 30 {
            break;
        }
    }

    let total_hops = hops.len() as u8;
    let packet_loss = hops.iter()
        .filter(|h| h.ip_address.is_none())
        .count() as f64 / total_hops.max(1) as f64 * 100.0;

    Ok(NetworkDiagnostics {
        target: target.to_string(),
        resolved_ip,
        hops,
        total_hops,
        packet_loss,
    })
}

fn print_traceroute_visualization(diag: &NetworkDiagnostics) {
    println!();
    println!("╔═══════════════════════════════════════════════════════════════════════════╗");
    println!("║  NETWORK ROUTE TO: {:^53} ║", diag.target);
    println!("╠═══════════════════════════════════════════════════════════════════════════╣");

    if let Some(ref ip) = diag.resolved_ip {
        println!("║  Resolved IP: {:^58} ║", ip);
    }
    println!("║  Total Hops: {:^3}    Packet Loss: {:>5.1}%                                ║",
        diag.total_hops, diag.packet_loss);
    println!("╠═══════════════════════════════════════════════════════════════════════════╣");

    // ASCII visualization header
    println!("║                                                                           ║");
    println!("║  YOUR PC                                                       TARGET    ║");
    println!("║    ┌──┐                                                        ┌──┐      ║");
    println!("║    │PC│                                                        │🌐│      ║");
    println!("║    └──┘                                                        └──┘      ║");
    println!("║      │                                                           │        ║");

    // Draw route
    let max_hops = diag.hops.len().min(15);
    let width_per_hop = 70 / max_hops.max(1);

    // Top line of route
    print!("║      ");
    for (i, hop) in diag.hops.iter().take(max_hops).enumerate() {
        if hop.is_target {
            print!("◉");
        } else if hop.ip_address.is_some() {
            print!("●");
        } else {
            print!("○");
        }
        if i < max_hops - 1 {
            for _ in 0..width_per_hop.saturating_sub(1) {
                print!("─");
            }
        }
    }
    println!("──→     ║");

    println!("║                                                                           ║");
    println!("╠═══════════════════════════════════════════════════════════════════════════╣");
    println!("║  HOP │ IP ADDRESS         │ RTT (ms)          │ STATUS                   ║");
    println!("╠══════╪════════════════════╪═══════════════════╪══════════════════════════╣");

    for hop in &diag.hops {
        let ip = hop.ip_address.as_deref().unwrap_or("* * *");
        let rtt = if hop.rtt_ms.is_empty() {
            "timeout".to_string()
        } else {
            let avg: f64 = hop.rtt_ms.iter().sum::<f64>() / hop.rtt_ms.len() as f64;
            format!("{:.2}", avg)
        };

        let status = if hop.is_target {
            "◉ TARGET"
        } else if hop.ip_address.is_none() {
            "○ No response"
        } else if hop.rtt_ms.iter().any(|&r| r > 100.0) {
            "● Slow"
        } else {
            "● OK"
        };

        println!("║  {:>3} │ {:^18} │ {:^17} │ {:^24} ║",
            hop.hop_number, ip, rtt, status);
    }

    println!("╚═══════════════════════════════════════════════════════════════════════════╝");

    // Legend
    println!();
    println!("Legend: ● Responding hop   ○ No response/timeout   ◉ Target reached");
    println!();
}

fn print_performance_metrics(metrics: &PerformanceMetrics, url: &str) {
    println!();
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  PERFORMANCE METRICS: {:^40} ║", url);
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║  HTTP Status: {:>3}                                            ║", metrics.status_code);
    println!("║  Response Size: {:>10} bytes                              ║", metrics.response_size);
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║                                                               ║");
    println!("║  TIMING BREAKDOWN                                             ║");
    println!("║  ─────────────────────────────────────────────────────────    ║");

    // Waterfall visualization
    let max_width = 40;
    let total = metrics.total_ms;

    let dns_width = ((metrics.dns_lookup_ms / total) * max_width as f64) as usize;
    let tcp_width = ((metrics.tcp_connect_ms / total) * max_width as f64) as usize;
    let ttfb_width = ((metrics.ttfb_ms / total) * max_width as f64) as usize;
    let download_width = ((metrics.download_ms / total) * max_width as f64) as usize;

    println!("║  DNS Lookup    │{:░<width$}│ {:>8.2}ms          ║", "", metrics.dns_lookup_ms, width = dns_width.max(1));
    println!("║  TCP Connect   │{:▒<width$}│ {:>8.2}ms          ║", "", metrics.tcp_connect_ms, width = tcp_width.max(1));
    println!("║  TTFB          │{:▓<width$}│ {:>8.2}ms          ║", "", metrics.ttfb_ms, width = ttfb_width.max(1));
    println!("║  Download      │{:█<width$}│ {:>8.2}ms          ║", "", metrics.download_ms, width = download_width.max(1));
    println!("║  ─────────────────────────────────────────────────────────    ║");
    println!("║  TOTAL         │{:█<width$}│ {:>8.2}ms          ║", "", metrics.total_ms, width = max_width);
    println!("║                                                               ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");

    // Rating
    let rating = if metrics.total_ms < 100.0 {
        ("★★★★★", "EXCELLENT", "Sub-100ms response!")
    } else if metrics.total_ms < 300.0 {
        ("★★★★☆", "VERY GOOD", "Fast response time")
    } else if metrics.total_ms < 500.0 {
        ("★★★☆☆", "GOOD", "Acceptable performance")
    } else if metrics.total_ms < 1000.0 {
        ("★★☆☆☆", "FAIR", "Could be improved")
    } else {
        ("★☆☆☆☆", "SLOW", "Needs optimization")
    };

    println!("║  Rating: {} {} - {}            ║", rating.0, rating.1, rating.2);
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();
}

fn run_full_performance_test(host: &str) {
    println!();
    println!("╔═══════════════════════════════════════════════════════════════════════════╗");
    println!("║     SOUTH CITY COMPUTER - Client Performance & Network Diagnostics        ║");
    println!("╠═══════════════════════════════════════════════════════════════════════════╣");
    println!("║  Target: {:^65} ║", host);
    println!("╚═══════════════════════════════════════════════════════════════════════════╝");

    // Test multiple endpoints
    let endpoints = vec![
        ("/", "Homepage"),
        ("/css/style.min.css", "Stylesheet"),
        ("/js/main.min.js", "JavaScript"),
        ("/images/logo.webp", "Logo Image"),
        ("/health", "Health Check"),
    ];

    println!();
    println!("Testing {} endpoints...", endpoints.len());

    let mut results: HashMap<String, PerformanceMetrics> = HashMap::new();

    for (path, name) in &endpoints {
        print!("  {} {}... ", name, path);
        match measure_endpoint(host, path, 80) {
            Ok(metrics) => {
                println!("{:.2}ms ✓", metrics.total_ms);
                results.insert(name.to_string(), metrics);
            }
            Err(e) => {
                println!("FAILED: {}", e);
            }
        }
    }

    // Print detailed results for homepage
    if let Some(metrics) = results.get("Homepage") {
        print_performance_metrics(metrics, &format!("http://{}/", host));
    }

    // Summary
    println!();
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  SUMMARY                                                      ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");

    let total_size: usize = results.values().map(|m| m.response_size).sum();
    let avg_time: f64 = results.values().map(|m| m.total_ms).sum::<f64>() / results.len().max(1) as f64;
    let successful = results.values().filter(|m| m.status_code == 200).count();

    println!("║  Endpoints tested: {:>5}                                     ║", endpoints.len());
    println!("║  Successful:       {:>5}                                     ║", successful);
    println!("║  Total data:       {:>10} bytes                          ║", total_size);
    println!("║  Average time:     {:>10.2} ms                            ║", avg_time);
    println!("╚═══════════════════════════════════════════════════════════════╝");
}

fn print_help() {
    println!("South City Computer - Client Performance & Network Diagnostics");
    println!();
    println!("USAGE:");
    println!("    scc-perf-client [OPTIONS] [COMMAND]");
    println!();
    println!("COMMANDS:");
    println!("    test <host>      Run full performance test (default)");
    println!("    trace <host>     Run traceroute and visualize network path");
    println!("    measure <url>    Measure single URL performance");
    println!("    help             Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("    scc-perf-client test southcitycomputer.com");
    println!("    scc-perf-client trace google.com");
    println!("    scc-perf-client measure http://localhost:9000/");
    println!();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "test" => {
            let host = args.get(2).map(|s| s.as_str()).unwrap_or(DEFAULT_TARGET);
            run_full_performance_test(host);
        }
        "trace" => {
            let target = args.get(2).map(|s| s.as_str()).unwrap_or(DEFAULT_TARGET);
            match run_traceroute(target) {
                Ok(diag) => print_traceroute_visualization(&diag),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        "measure" => {
            if args.len() < 3 {
                eprintln!("Usage: scc-perf-client measure <url>");
                return;
            }
            let url = &args[2];
            // Parse URL (simple)
            let url = url.trim_start_matches("http://").trim_start_matches("https://");
            let (host, path) = if let Some(pos) = url.find('/') {
                (&url[..pos], &url[pos..])
            } else {
                (url, "/")
            };

            match measure_endpoint(host, path, 80) {
                Ok(metrics) => print_performance_metrics(&metrics, &args[2]),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        "help" | "--help" | "-h" => {
            print_help();
        }
        _ => {
            // Assume it's a host to test
            run_full_performance_test(&args[1]);
        }
    }
}

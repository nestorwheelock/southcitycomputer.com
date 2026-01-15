// South City Computer - Server Performance Benchmark
// Measures response times, throughput, and resource usage

use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

const DEFAULT_HOST: &str = "127.0.0.1:9000";
const WARMUP_REQUESTS: u32 = 10;

#[derive(Clone)]
struct BenchmarkConfig {
    host: String,
    requests: u32,
    concurrency: u32,
    warmup: bool,
    verbose: bool,
}

#[derive(Default)]
struct BenchmarkResults {
    total_requests: AtomicU64,
    successful_requests: AtomicU64,
    failed_requests: AtomicU64,
    total_bytes: AtomicU64,
    min_latency_us: AtomicU64,
    max_latency_us: AtomicU64,
    total_latency_us: AtomicU64,
}

impl BenchmarkResults {
    fn new() -> Self {
        Self {
            min_latency_us: AtomicU64::new(u64::MAX),
            ..Default::default()
        }
    }

    fn record_success(&self, latency_us: u64, bytes: u64) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        self.successful_requests.fetch_add(1, Ordering::Relaxed);
        self.total_bytes.fetch_add(bytes, Ordering::Relaxed);
        self.total_latency_us.fetch_add(latency_us, Ordering::Relaxed);

        // Update min
        let mut current_min = self.min_latency_us.load(Ordering::Relaxed);
        while latency_us < current_min {
            match self.min_latency_us.compare_exchange_weak(
                current_min,
                latency_us,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(x) => current_min = x,
            }
        }

        // Update max
        let mut current_max = self.max_latency_us.load(Ordering::Relaxed);
        while latency_us > current_max {
            match self.max_latency_us.compare_exchange_weak(
                current_max,
                latency_us,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(x) => current_max = x,
            }
        }
    }

    fn record_failure(&self) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        self.failed_requests.fetch_add(1, Ordering::Relaxed);
    }
}

fn make_request(host: &str, path: &str) -> Result<(u64, Vec<u8>), String> {
    let start = Instant::now();

    let mut stream = TcpStream::connect(host)
        .map_err(|e| format!("Connection failed: {}", e))?;

    stream.set_read_timeout(Some(Duration::from_secs(30)))
        .map_err(|e| format!("Set timeout failed: {}", e))?;

    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        path, host
    );

    stream.write_all(request.as_bytes())
        .map_err(|e| format!("Write failed: {}", e))?;

    let mut response = Vec::new();
    stream.read_to_end(&mut response)
        .map_err(|e| format!("Read failed: {}", e))?;

    let latency = start.elapsed().as_micros() as u64;

    // Check for HTTP 200
    let response_str = String::from_utf8_lossy(&response);
    if !response_str.starts_with("HTTP/1.1 200") && !response_str.starts_with("HTTP/1.0 200") {
        return Err(format!("Non-200 response: {}", response_str.lines().next().unwrap_or("")));
    }

    Ok((latency, response))
}

fn run_benchmark(config: &BenchmarkConfig, path: &str, results: Arc<BenchmarkResults>, running: Arc<AtomicBool>) {
    let requests_per_thread = config.requests / config.concurrency;
    let mut handles = vec![];

    for _ in 0..config.concurrency {
        let host = config.host.clone();
        let path = path.to_string();
        let results = Arc::clone(&results);
        let running = Arc::clone(&running);

        let handle = thread::spawn(move || {
            for _ in 0..requests_per_thread {
                if !running.load(Ordering::Relaxed) {
                    break;
                }

                match make_request(&host, &path) {
                    Ok((latency, response)) => {
                        results.record_success(latency, response.len() as u64);
                    }
                    Err(_) => {
                        results.record_failure();
                    }
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn format_bytes(bytes: u64) -> String {
    if bytes >= 1_000_000_000 {
        format!("{:.2} GB", bytes as f64 / 1_000_000_000.0)
    } else if bytes >= 1_000_000 {
        format!("{:.2} MB", bytes as f64 / 1_000_000.0)
    } else if bytes >= 1_000 {
        format!("{:.2} KB", bytes as f64 / 1_000.0)
    } else {
        format!("{} B", bytes)
    }
}

fn format_latency(us: u64) -> String {
    if us >= 1_000_000 {
        format!("{:.2}s", us as f64 / 1_000_000.0)
    } else if us >= 1_000 {
        format!("{:.2}ms", us as f64 / 1_000.0)
    } else {
        format!("{}μs", us)
    }
}

fn print_results(name: &str, results: &BenchmarkResults, duration: Duration) {
    let total = results.total_requests.load(Ordering::Relaxed);
    let successful = results.successful_requests.load(Ordering::Relaxed);
    let failed = results.failed_requests.load(Ordering::Relaxed);
    let bytes = results.total_bytes.load(Ordering::Relaxed);
    let min_us = results.min_latency_us.load(Ordering::Relaxed);
    let max_us = results.max_latency_us.load(Ordering::Relaxed);
    let total_latency = results.total_latency_us.load(Ordering::Relaxed);

    let avg_latency = if successful > 0 { total_latency / successful } else { 0 };
    let throughput = if duration.as_secs_f64() > 0.0 {
        successful as f64 / duration.as_secs_f64()
    } else {
        0.0
    };

    println!();
    println!("┌─────────────────────────────────────────────────────────────┐");
    println!("│  {} {:>45} │", name, "");
    println!("├─────────────────────────────────────────────────────────────┤");
    println!("│  Requests:     {:>10} total, {:>10} ok, {:>6} fail │", total, successful, failed);
    println!("│  Throughput:   {:>10.2} req/s                           │", throughput);
    println!("│  Data:         {:>10}                                  │", format_bytes(bytes));
    println!("├─────────────────────────────────────────────────────────────┤");
    println!("│  Latency:                                                   │");
    println!("│    Min:        {:>10}                                  │", format_latency(min_us));
    println!("│    Avg:        {:>10}                                  │", format_latency(avg_latency));
    println!("│    Max:        {:>10}                                  │", format_latency(max_us));
    println!("└─────────────────────────────────────────────────────────────┘");
}

fn benchmark_endpoint(config: &BenchmarkConfig, name: &str, path: &str) {
    // Warmup
    if config.warmup {
        if config.verbose {
            println!("  Warming up {} ({} requests)...", name, WARMUP_REQUESTS);
        }
        for _ in 0..WARMUP_REQUESTS {
            let _ = make_request(&config.host, path);
        }
    }

    if config.verbose {
        println!("  Benchmarking {} ({} requests, {} concurrent)...",
            name, config.requests, config.concurrency);
    }

    let results = Arc::new(BenchmarkResults::new());
    let running = Arc::new(AtomicBool::new(true));

    let start = Instant::now();
    run_benchmark(config, path, Arc::clone(&results), Arc::clone(&running));
    let duration = start.elapsed();

    print_results(name, &results, duration);
}

fn run_full_suite(config: &BenchmarkConfig) {
    println!();
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║     SOUTH CITY COMPUTER - Performance Benchmark Suite         ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║  Target: {}                                          ║", config.host);
    println!("║  Requests per test: {:>6}                                   ║", config.requests);
    println!("║  Concurrency: {:>6}                                         ║", config.concurrency);
    println!("╚═══════════════════════════════════════════════════════════════╝");

    // Test endpoints
    let endpoints = vec![
        ("Homepage (HTML)", "/"),
        ("Health Check (JSON)", "/health"),
        ("CSS Stylesheet", "/css/style.min.css"),
        ("JavaScript", "/js/main.min.js"),
        ("Logo (small image)", "/images/logo.webp"),
        ("Storefront (medium image)", "/images/storefront.webp"),
        ("Service Page", "/services/computer-repair.html"),
    ];

    for (name, path) in endpoints {
        benchmark_endpoint(config, name, path);
    }

    // Full page simulation
    println!();
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  Full Page Load Simulation (Above-the-fold assets)            ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");

    let critical_assets = vec![
        "/",
        "/css/style.min.css",
        "/js/main.min.js",
        "/images/logo.webp",
        "/images/storefront.webp",
    ];

    let start = Instant::now();
    let mut total_bytes = 0u64;
    let mut all_success = true;

    for path in &critical_assets {
        match make_request(&config.host, path) {
            Ok((_, data)) => {
                total_bytes += data.len() as u64;
            }
            Err(e) => {
                eprintln!("  Failed to load {}: {}", path, e);
                all_success = false;
            }
        }
    }

    let page_load_time = start.elapsed();

    println!();
    println!("┌─────────────────────────────────────────────────────────────┐");
    println!("│  Full Page Load Results                                     │");
    println!("├─────────────────────────────────────────────────────────────┤");
    println!("│  Assets loaded:   {:>5}                                    │", critical_assets.len());
    println!("│  Total size:      {:>10}                               │", format_bytes(total_bytes));
    println!("│  Load time:       {:>10}                               │", format_latency(page_load_time.as_micros() as u64));
    println!("│  Status:          {:>10}                               │", if all_success { "SUCCESS" } else { "FAILED" });
    println!("└─────────────────────────────────────────────────────────────┘");

    // Summary
    println!();
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  BENCHMARK COMPLETE                                           ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    if page_load_time.as_millis() < 100 {
        println!("║  Rating: ★★★★★ EXCELLENT (<100ms full page)                  ║");
    } else if page_load_time.as_millis() < 500 {
        println!("║  Rating: ★★★★☆ VERY GOOD (<500ms full page)                  ║");
    } else if page_load_time.as_millis() < 1000 {
        println!("║  Rating: ★★★☆☆ GOOD (<1s full page)                          ║");
    } else if page_load_time.as_millis() < 2500 {
        println!("║  Rating: ★★☆☆☆ ACCEPTABLE (<2.5s Core Web Vitals)            ║");
    } else {
        println!("║  Rating: ★☆☆☆☆ NEEDS IMPROVEMENT (>2.5s)                     ║");
    }
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();
}

fn run_quick_test(config: &BenchmarkConfig) {
    println!();
    println!("Quick connectivity test to {}...", config.host);

    match make_request(&config.host, "/health") {
        Ok((latency, _)) => {
            println!("✓ Server responding - latency: {}", format_latency(latency));
        }
        Err(e) => {
            println!("✗ Server not responding: {}", e);
            std::process::exit(1);
        }
    }
}

fn print_help() {
    println!("South City Computer - Performance Benchmark Tool");
    println!();
    println!("USAGE:");
    println!("    scc-benchmark [OPTIONS] [COMMAND]");
    println!();
    println!("COMMANDS:");
    println!("    full        Run full benchmark suite (default)");
    println!("    quick       Quick connectivity test");
    println!("    endpoint    Benchmark a single endpoint");
    println!("    help        Show this help message");
    println!();
    println!("OPTIONS:");
    println!("    -h, --host <HOST>       Target host (default: 127.0.0.1:9000)");
    println!("    -n, --requests <N>      Number of requests per test (default: 100)");
    println!("    -c, --concurrency <N>   Concurrent connections (default: 10)");
    println!("    -w, --no-warmup         Skip warmup requests");
    println!("    -v, --verbose           Verbose output");
    println!();
    println!("EXAMPLES:");
    println!("    scc-benchmark                           # Full suite on localhost:9000");
    println!("    scc-benchmark -h 192.168.1.100:9000     # Test remote server");
    println!("    scc-benchmark -n 1000 -c 50             # Heavy load test");
    println!("    scc-benchmark quick                     # Quick connectivity check");
    println!("    scc-benchmark endpoint /images/logo.webp  # Single endpoint");
    println!();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut config = BenchmarkConfig {
        host: DEFAULT_HOST.to_string(),
        requests: 100,
        concurrency: 10,
        warmup: true,
        verbose: false,
    };

    let mut command = "full".to_string();
    let mut endpoint_path = String::new();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--host" => {
                i += 1;
                if i < args.len() {
                    config.host = args[i].clone();
                }
            }
            "-n" | "--requests" => {
                i += 1;
                if i < args.len() {
                    config.requests = args[i].parse().unwrap_or(100);
                }
            }
            "-c" | "--concurrency" => {
                i += 1;
                if i < args.len() {
                    config.concurrency = args[i].parse().unwrap_or(10);
                }
            }
            "-w" | "--no-warmup" => {
                config.warmup = false;
            }
            "-v" | "--verbose" => {
                config.verbose = true;
            }
            "full" => command = "full".to_string(),
            "quick" => command = "quick".to_string(),
            "help" | "--help" => {
                print_help();
                return;
            }
            "endpoint" => {
                command = "endpoint".to_string();
                i += 1;
                if i < args.len() {
                    endpoint_path = args[i].clone();
                }
            }
            _ => {
                // Check if it looks like a path
                if args[i].starts_with('/') {
                    endpoint_path = args[i].clone();
                    if command != "endpoint" {
                        command = "endpoint".to_string();
                    }
                }
            }
        }
        i += 1;
    }

    match command.as_str() {
        "full" => run_full_suite(&config),
        "quick" => run_quick_test(&config),
        "endpoint" => {
            if endpoint_path.is_empty() {
                eprintln!("Error: endpoint command requires a path");
                eprintln!("Usage: scc-benchmark endpoint /path/to/resource");
                std::process::exit(1);
            }
            benchmark_endpoint(&config, &endpoint_path, &endpoint_path);
        }
        _ => {
            print_help();
        }
    }
}

# South City Computer Website

A high-performance, single-binary web server built with Rust. Achieves 46x faster page loads than traditional WordPress setups through embedded assets and memory-resident architecture.

## Philosophy

This project demonstrates that there are better alternatives to WordPress for many websites. Our approach:

- **Monolithic binary**: All HTML, CSS, JS, and images embedded in a single executable
- **Decoupled data storage**: Contact form submissions saved to CSV files that can be picked up by external processes (database import, email notification, ticket creation) - keeping business logic separate from the web attack surface
- **Performance first**: Sub-millisecond response times, 26,000-58,000 req/s throughput
- **Simple deployment**: Copy one file, run it

## Quick Start

```bash
# Build the production server
cd contact-handler
cargo build --release

# Run it
./target/release/scc-server
# Server running at http://127.0.0.1:9000
```

## Binaries

| Binary | Purpose | Usage |
|--------|---------|-------|
| `scc-server` | Production web server | Embedded assets, optimized for speed |
| `scc-dev` | Development server | Reads files from disk, instant refresh |
| `scc-desktop` | Desktop app wrapper | Native window with embedded server |
| `scc-benchmark` | Server-side benchmarks | Measure throughput and latency |
| `scc-perf-client` | Client-side diagnostics | Network timing, traceroute visualization |

### Production Server (scc-server)

The main production binary with all assets embedded in memory.

```bash
cargo build --release
./target/release/scc-server

# Options:
# PORT=8080 ./target/release/scc-server  # Custom port
```

Features:
- All static assets served from memory (zero disk I/O)
- Gzip/Brotli compression for text assets
- Contact form API with CSV storage
- Health check endpoint at `/health`

### Development Server (scc-dev)

Reads files from disk for rapid iteration. No recompilation needed when changing HTML/CSS/JS.

```bash
cargo build
./target/debug/scc-dev

# Options:
# STATIC_DIR=/path/to/assets ./target/debug/scc-dev
# PORT=3000 ./target/debug/scc-dev
```

Features:
- Hot reload: Edit files, refresh browser, see changes
- No-cache headers for immediate updates
- Same API endpoints as production
- Faster compile times (no asset embedding)

### Desktop App (scc-desktop)

Native desktop application using the Wry/Tao WebView.

```bash
cargo build --release --features desktop
./target/release/scc-desktop
```

Requires the `desktop` feature flag and appropriate system libraries.

### Benchmark Tool (scc-benchmark)

Measure server performance with concurrent load testing.

```bash
cargo build --release
./target/release/scc-benchmark

# Commands:
scc-benchmark                      # Full benchmark suite
scc-benchmark quick                # Quick connectivity test
scc-benchmark endpoint /api/health # Test specific endpoint

# Options:
scc-benchmark -h 192.168.1.100:9000  # Test remote server
scc-benchmark -n 1000 -c 50          # 1000 requests, 50 concurrent
scc-benchmark -v                     # Verbose output
```

### Performance Client (scc-perf-client)

Client-side performance diagnostics with network visualization.

```bash
cargo build --release
./target/release/scc-perf-client

# Commands:
scc-perf-client test https://example.com      # Full performance test
scc-perf-client measure https://example.com   # Timing breakdown
scc-perf-client trace example.com             # Traceroute visualization

# Options:
scc-perf-client test -n 10 https://example.com  # 10 requests
scc-perf-client trace -m 20 example.com         # Max 20 hops
```

## Project Structure

```
southcitycomputer.com/
├── index.html              # Main website
├── css/
│   ├── style.css          # Source CSS
│   └── style.min.css      # Minified (embedded in binary)
├── js/
│   ├── main.js            # Source JavaScript
│   └── main.min.js        # Minified (embedded in binary)
├── images/                 # WebP optimized images
├── services/              # Service pages
├── app/                   # Mobile app download page
├── contact-handler/       # Rust server
│   ├── src/
│   │   ├── main.rs        # Production server
│   │   ├── dev_server.rs  # Development server
│   │   ├── desktop.rs     # Desktop app
│   │   ├── benchmark.rs   # Server benchmarks
│   │   └── perf_client.rs # Performance client
│   └── Cargo.toml
├── android-app/           # Android WebView app
├── PERFORMANCE_TESTING.md # Benchmark results
├── WHITEPAPER.md          # Technical deep-dive
└── DEVELOPER.md           # Developer guide
```

## Development Workflow

### 1. Use Development Server for UI Changes

```bash
# Terminal 1: Run dev server
cd contact-handler
cargo run --bin scc-dev

# Terminal 2: Edit files
# Changes to HTML/CSS/JS appear immediately on browser refresh
```

### 2. Build Production Binary

```bash
cd contact-handler
cargo build --release

# Test it works
./target/release/scc-server &
curl http://localhost:9000/health
```

### 3. Run Benchmarks

```bash
# Quick check
./target/release/scc-benchmark quick

# Full suite
./target/release/scc-benchmark
```

## Test-Driven Development

This project follows TDD practices:

1. **Write failing tests first** - Define expected behavior
2. **Write minimal code** - Make tests pass
3. **Refactor** - Clean up while tests stay green

### Running Tests

```bash
cd contact-handler
cargo test

# With output
cargo test -- --nocapture

# Specific test
cargo test test_health_endpoint
```

### Test Categories

- **Unit tests**: Individual function behavior
- **Integration tests**: API endpoint behavior
- **Benchmark tests**: Performance regression detection

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/` | GET | Main website |
| `/health` | GET | Health check (JSON) |
| `/api/contact` | POST | Submit contact form |
| `/view/contacts` | GET | View submissions (requires auth) |
| `/*` | GET | Static assets |

### Contact Form API

```bash
curl -X POST http://localhost:9000/api/contact \
  -H "Content-Type: application/json" \
  -d '{"name":"Test","email":"test@example.com","message":"Hello"}'
```

Response:
```json
{"success": true, "message": "Contact saved successfully", "id": "uuid"}
```

## Data Storage

Contact submissions are stored in `contacts.csv`:

```csv
id,timestamp,name,email,phone,message,service
uuid,2026-01-15T10:30:00,John Doe,john@example.com,555-1234,Hello,repair
```

### Decoupled Processing

The CSV file acts as a queue. External processes can:

1. Read new entries
2. Import to database
3. Send email notifications
4. Create support tickets
5. Clear processed entries

This keeps sensitive operations (email, database) separate from the web server, reducing attack surface.

## Security

### Current

- Input validation on all form fields
- CSV storage (no SQL injection possible)
- Minimal dependencies
- No user sessions (stateless API)

### Roadmap

- [ ] Encrypted CSV storage (AES-256)
- [ ] Password hashing for admin accounts (Argon2)
- [ ] Rate limiting on contact form
- [ ] CSRF tokens for forms

## Performance

Documented in [PERFORMANCE_TESTING.md](PERFORMANCE_TESTING.md) and [WHITEPAPER.md](WHITEPAPER.md).

### Key Metrics (January 2026)

| Metric | Value |
|--------|-------|
| Health check throughput | 58,085 req/s |
| Homepage throughput | 26,795 req/s |
| Full page load (5 assets) | 1.60ms |
| Average latency | 36-382μs |
| Binary size | 19MB |
| Memory usage | ~1.1MB RSS |

### Benchmark Results

```
Endpoint          Throughput    Avg Latency    Data Transfer
─────────────────────────────────────────────────────────────
Health Check      58,085 req/s      61μs       12.5 KB
Homepage          26,795 req/s     122μs       1.99 MB
CSS Stylesheet    41,100 req/s      71μs       1.66 MB
JavaScript        45,406 req/s      66μs       1.24 MB
Logo (5KB)        39,287 req/s      65μs       270 KB
Storefront (128KB) 3,676 req/s     206μs       6.43 MB
─────────────────────────────────────────────────────────────
Rating: ★★★★★ EXCELLENT (<100ms full page)
```

### Optimization Summary

- WebP images: 78% smaller than JPEG
- Minified CSS/JS: 24% smaller
- Memory-resident assets: Zero disk I/O
- Lazy loading: 90% initial payload reduction
- All assets embedded in single binary

## Deployment

### Single Binary

```bash
# Build
cargo build --release

# Deploy
scp target/release/scc-server user@server:/opt/scc/

# Run
ssh user@server "/opt/scc/scc-server"
```

### Docker

```bash
docker build -t scc-server .
docker run -p 9000:9000 scc-server
```

### Systemd Service

```ini
[Unit]
Description=South City Computer Web Server
After=network.target

[Service]
Type=simple
ExecStart=/opt/scc/scc-server
Restart=always
User=www-data

[Install]
WantedBy=multi-user.target
```

## Minification

CSS and JS are minified using clean-css and terser:

```bash
# Install tools
npm install

# Minify
npx cleancss -o css/style.min.css css/style.css
npx terser js/main.js -o js/main.min.js --compress --mangle
```

After minification, rebuild the production binary to embed updated files.

## License

Dual licensed:

- **GPLv3** for non-commercial and open source use - see [LICENSE-GPL](LICENSE-GPL)
- **Commercial License** for business use - see [LICENSE-COMMERCIAL](LICENSE-COMMERCIAL)

See [LICENSE](LICENSE) for details on which license applies to your use case.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Write tests first (TDD)
4. Make your changes
5. Ensure all tests pass
6. Submit a pull request

## Documentation

- [DEVELOPER.md](DEVELOPER.md) - Developer guide and architecture
- [PERFORMANCE_TESTING.md](PERFORMANCE_TESTING.md) - Benchmark methodology and results
- [WHITEPAPER.md](WHITEPAPER.md) - Technical deep-dive on performance optimization
- [ROADMAP.md](ROADMAP.md) - Future plans and user stories

## Contact

South City Computer
- Website: https://southcitycomputer.com
- Email: info@southcitycomputer.com

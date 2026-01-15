# Developer Guide

Technical documentation for contributing to the South City Computer website project.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                     Single Rust Binary                       │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │   Actix     │  │   Embedded  │  │      API            │  │
│  │   Web       │  │   Assets    │  │      Handlers       │  │
│  │   Server    │  │   (rust-    │  │                     │  │
│  │             │  │   embed)    │  │  /api/contact       │  │
│  │  Port 9000  │  │             │  │  /view/contacts     │  │
│  │             │  │  HTML/CSS   │  │  /health            │  │
│  │             │  │  JS/Images  │  │                     │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                     File System                              │
│  ┌──────────────┐                                           │
│  │ contacts.csv │  ← Decoupled from web server              │
│  │              │  ← Can be processed by external tools     │
│  └──────────────┘                                           │
└─────────────────────────────────────────────────────────────┘
```

### Design Principles

1. **Single Binary Deployment**
   - All assets compiled into the executable
   - No external file dependencies at runtime
   - Simplifies deployment and reduces failure modes

2. **Memory-Resident Assets**
   - Static files served from RAM
   - Zero disk I/O for static content
   - Consistent sub-millisecond response times

3. **Decoupled Data Storage**
   - Contact form writes to CSV file
   - External processes handle email, database, tickets
   - Reduces web server attack surface

4. **Development/Production Parity**
   - Same API in both dev and prod servers
   - Dev server just reads from disk instead of memory

## Code Organization

```
contact-handler/src/
├── main.rs           # Production server with embedded assets
├── dev_server.rs     # Development server (disk reads)
├── desktop.rs        # Desktop app with embedded server
├── benchmark.rs      # Server-side performance testing
└── perf_client.rs    # Client-side network diagnostics
```

### main.rs - Production Server

Key components:

```rust
// Asset embedding
#[derive(RustEmbed)]
#[folder = "../"]
#[include = "*.html"]
#[include = "css/*.min.css"]
#[include = "js/*.min.js"]
#[include = "images/*.webp"]
struct Asset;

// Static file handler
async fn serve_static(path: web::Path<String>) -> impl Responder {
    // Serve from embedded assets with proper MIME types
}

// Contact form API
async fn submit_contact(form: web::Json<ContactForm>) -> impl Responder {
    // Validate, generate UUID, write to CSV
}
```

### dev_server.rs - Development Server

Differences from production:

- Reads files from disk on each request
- No caching headers (always fresh)
- Faster compilation (no embedding step)
- Same API endpoints for consistency

## Test-Driven Development

### TDD Cycle

```
1. Write a failing test
   └── cargo test -- --nocapture
       └── Test fails (expected)

2. Write minimal code to pass
   └── cargo test
       └── Test passes

3. Refactor
   └── cargo test
       └── Still passes
```

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact_validation() {
        let form = ContactForm {
            name: "".to_string(),  // Invalid: empty
            email: "test@example.com".to_string(),
            message: "Hello".to_string(),
        };
        assert!(validate_contact(&form).is_err());
    }

    #[test]
    fn test_valid_contact() {
        let form = ContactForm {
            name: "John".to_string(),
            email: "john@example.com".to_string(),
            message: "Hello".to_string(),
        };
        assert!(validate_contact(&form).is_ok());
    }
}
```

### Test Categories

| Type | Location | Purpose |
|------|----------|---------|
| Unit | `src/*.rs` | Function-level behavior |
| Integration | `tests/` | API endpoint behavior |
| Benchmark | `benches/` | Performance regression |

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_contact_validation

# With output
cargo test -- --nocapture

# Only doc tests
cargo test --doc
```

## Development Workflow

### 1. Setup

```bash
# Clone repo
git clone https://github.com/southcitycomputer/southcitycomputer.com

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node (for minification)
# Use your preferred method (nvm, apt, etc.)

# Install minification tools
npm install
```

### 2. Development Cycle

```bash
# Start dev server (watches files)
cd contact-handler
cargo run --bin scc-dev

# In another terminal, edit files
# HTML/CSS/JS changes appear on browser refresh
```

### 3. Building Production

```bash
# Minify assets first
npx cleancss -o css/style.min.css css/style.css
npx terser js/main.js -o js/main.min.js --compress --mangle

# Build release binary
cd contact-handler
cargo build --release

# Binary at: target/release/scc-server
```

### 4. Testing

```bash
# Run all tests
cargo test

# Run with benchmarks
cargo test --release

# Check specific functionality
cargo test health
cargo test contact
```

## Adding New Features

### Adding a New API Endpoint

1. **Write tests first**

```rust
#[cfg(test)]
mod tests {
    #[actix_web::test]
    async fn test_new_endpoint() {
        let app = test::init_service(
            App::new().route("/api/new", web::get().to(new_handler))
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/new")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
```

2. **Run test (should fail)**

```bash
cargo test test_new_endpoint
# FAILED - handler doesn't exist
```

3. **Implement handler**

```rust
async fn new_handler() -> impl Responder {
    HttpResponse::Ok().json(json!({"status": "ok"}))
}
```

4. **Run test (should pass)**

```bash
cargo test test_new_endpoint
# PASSED
```

5. **Add to router**

```rust
.route("/api/new", web::get().to(new_handler))
```

### Adding New Static Assets

1. Place files in appropriate directory
2. For production, minify if needed
3. Update `RustEmbed` includes if using new file type
4. Rebuild production binary

```rust
#[derive(RustEmbed)]
#[folder = "../"]
#[include = "*.html"]
#[include = "css/*.min.css"]
#[include = "js/*.min.js"]
#[include = "images/*.webp"]
#[include = "fonts/*.woff2"]  // New file type
struct Asset;
```

## Code Style

### Rust

- Run `cargo fmt` before committing
- Run `cargo clippy` to catch common issues
- Keep functions focused and small
- Document public APIs

```rust
/// Validates a contact form submission.
///
/// # Errors
/// Returns an error if name is empty or email is invalid.
pub fn validate_contact(form: &ContactForm) -> Result<(), ValidationError> {
    // ...
}
```

### HTML/CSS/JS

- Semantic HTML5
- Mobile-first responsive CSS
- Vanilla JavaScript (no frameworks)
- Progressive enhancement

## Debugging

### Server Logs

```bash
# Development - logs to stdout
cargo run --bin scc-dev 2>&1 | tee server.log

# Production - redirect to file
./scc-server > server.log 2>&1
```

### Common Issues

**Binary too large?**
- Check for debug symbols: `strip target/release/scc-server`
- Verify only `.min.css`/`.min.js` are included

**Assets not updating?**
- Rebuild binary after changing embedded files
- Clear browser cache
- Use dev server for iteration

**Contact form not working?**
- Check `contacts.csv` is writable
- Verify CORS headers if cross-origin
- Check browser console for errors

## Performance Testing

### Quick Validation

```bash
# Start server
./target/release/scc-server &

# Quick check
./target/release/scc-benchmark quick

# Full suite
./target/release/scc-benchmark -n 100 -c 10
```

### Regression Detection

Before merging changes, compare performance:

```bash
# Baseline (current main)
git checkout main
cargo build --release
./target/release/scc-benchmark > baseline.txt

# Your changes
git checkout feature-branch
cargo build --release
./target/release/scc-benchmark > feature.txt

# Compare
diff baseline.txt feature.txt
```

## Dependencies

Core dependencies (kept minimal):

| Crate | Purpose |
|-------|---------|
| actix-web | HTTP server framework |
| actix-cors | CORS middleware |
| rust-embed | Asset embedding |
| serde | Serialization |
| uuid | Unique IDs |
| chrono | Timestamps |

Optional:
| Crate | Purpose | Feature |
|-------|---------|---------|
| wry | WebView | `desktop` |
| tao | Window management | `desktop` |

## Security Considerations

### Input Validation

Always validate user input:

```rust
fn validate_email(email: &str) -> bool {
    // Basic validation - consider using validator crate for production
    email.contains('@') && email.contains('.')
}

fn sanitize_input(input: &str) -> String {
    // Remove potentially dangerous characters
    input.chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace() || ".,!?@-_".contains(*c))
        .collect()
}
```

### File System Access

- CSV files written with restricted permissions
- No user-controlled file paths
- Validate file operations

### Future Security Improvements

See [ROADMAP.md](ROADMAP.md) for planned security enhancements including encrypted storage and password hashing.

## Getting Help

- Open an issue on GitHub
- Check existing documentation
- Review test files for usage examples

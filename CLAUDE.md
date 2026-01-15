# South City Computer Development Instructions

You are developing a Rust-based website for South City Computer. Follow these instructions exactly.

---

## PROJECT CONTEXT

This is a single-page website with a Rust backend (actix-web) that handles:
- Static file serving (HTML, CSS, JS, images)
- Contact form submission (saves to CSV)
- Admin panel for viewing contacts (Basic Auth)

**Tech Stack:**
- Backend: Rust (actix-web, actix-files)
- Frontend: Static HTML/CSS/JS
- Data: CSV flat files
- Deployment: Bare metal with systemd + nginx

---

## PHASE 1: SPEC (When Asked to Add Features)

When asked to add a new feature, FIRST create planning documents before any code.

### Create Feature Spec

```markdown
# Feature: [Name]

## What
[One paragraph: what this feature provides]

## Why
[One paragraph: what problem it solves]

## How
[One paragraph: technical approach]

## Success Criteria
- [ ] [Measurable outcome 1]
- [ ] [Measurable outcome 2]

## Scope

**IN SCOPE:**
- [Feature 1]
- [Feature 2]

**OUT OF SCOPE:**
- [Excluded feature 1]
```

---

## PHASE 2: TDD WORKFLOW

### TDD Stop Gate

BEFORE writing ANY implementation code, output this confirmation:

```
=== TDD STOP GATE ===
Feature: [name]
[ ] I have read the feature spec
[ ] I am writing TESTS FIRST
[ ] Tests will fail because implementation doesn't exist
=== PROCEEDING WITH FAILING TESTS ===
```

### Write Failing Tests First

1. Create test file or add test cases
2. Run `cargo test` and SHOW the failing output
3. Confirm tests fail because code doesn't exist

### Implement Minimal Code

1. Write the minimum code to make ONE test pass
2. Run `cargo test` and show output
3. Repeat until all tests pass

### Output Completion

```
=== TDD CYCLE COMPLETE ===
Tests written BEFORE implementation: YES
All tests passing: [X/X]
=== READY FOR COMMIT ===
```

---

## PHASE 3: RUST PATTERNS

### Project Structure

```
southcitycomputer.com/
├── index.html
├── css/style.css
├── js/main.js
├── images/
├── contact-handler/
│   ├── Cargo.toml
│   ├── src/
│   │   └── main.rs
│   ├── accounts.txt
│   └── contacts.csv
└── scripts/
    ├── deploy.conf
    └── deploy.sh
```

### Code Patterns

**Pattern 1: Error Handling**
```rust
use actix_web::{HttpResponse, error::ResponseError};

#[derive(Debug)]
struct AppError(String);

impl ResponseError for AppError {}
```

**Pattern 2: JSON Response**
```rust
#[derive(Serialize)]
struct ApiResponse {
    success: bool,
    message: String,
}
```

**Pattern 3: CSV Escaping**
```rust
fn escape_csv_field(field: &str) -> String {
    field.replace(',', "\\,").replace('\n', " ")
}
```

**Pattern 4: Basic Auth**
```rust
fn check_auth(req: &HttpRequest) -> bool {
    // Parse Authorization header
    // Decode base64
    // Compare against accounts.txt
}
```

---

## PHASE 4: TESTING

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_escaping() {
        assert_eq!(escape_csv_field("hello,world"), "hello\\,world");
    }

    #[actix_web::test]
    async fn test_health_endpoint() {
        // Test HTTP endpoints
    }
}
```

### Run Tests

```bash
cd contact-handler
cargo test
```

---

## PHASE 5: DEPLOYMENT

### Deploy Commands

```bash
./scripts/deploy.sh deploy   # Build + upload + restart
./scripts/deploy.sh status   # Check service status
./scripts/deploy.sh logs     # View logs
./scripts/deploy.sh nginx    # Setup nginx + SSL
```

### Server Details

- Host: 108.61.224.251
- Service: southcitycomputer (systemd)
- Port: 9000 (internal), 80/443 (nginx)
- Path: /root/southcitycomputer

---

## PHASE 6: GIT WORKFLOW

### Commit Messages

Use conventional commit format:

```
feat(contact): add PDF download for confirmations

- Add unique ID generation for submissions
- Add /view/:id endpoint
- Add PDF generation with printpdf crate
```

Prefixes:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `refactor`: Code change that doesn't add feature or fix bug

### No AI Attribution

NEVER include:
- "Generated with Claude"
- "Co-Authored-By: Claude"
- Any AI/assistant references

This is a security requirement.

---

## CHECKLISTS

### New Feature Checklist

```
□ Feature spec created
□ TDD Stop Gate confirmed
□ Tests written BEFORE implementation
□ All tests passing
□ Code compiles without warnings
□ Manual testing complete
□ Deployed to production
□ README updated if needed
```

### Pre-Commit Checklist

```
□ cargo test passes
□ cargo clippy has no warnings
□ cargo fmt applied
□ Commit message is conventional format
```

---

## QUICK REFERENCE

### Build & Test

```bash
cd contact-handler
cargo build --release
cargo test
cargo clippy
cargo fmt
```

### Local Development

```bash
cd contact-handler
cargo run
# Server at http://localhost:9000
```

### Deploy

```bash
./scripts/deploy.sh deploy
./scripts/deploy.sh status
```

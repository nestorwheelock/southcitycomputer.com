# Roadmap & User Stories

## Vision

Provide an open-source alternative to WordPress that demonstrates how simple, fast, and maintainable a small business website can be. Share knowledge about performance optimization and decoupled architecture.

---

## User Stories

### Website Visitors

**S-001: Fast Page Load**
> As a visitor, I want the website to load instantly so that I can find information quickly without waiting.

Acceptance Criteria:
- [x] Full page load under 100ms (localhost)
- [x] Above-fold content under 50ms
- [x] All images optimized (WebP)
- [x] Lazy loading for below-fold images

**S-002: Contact Form**
> As a potential customer, I want to submit a contact form so that I can request services or ask questions.

Acceptance Criteria:
- [x] Form validates input before submission
- [x] Clear success/error messages
- [x] Form data saved securely
- [ ] Email confirmation sent (future)

**S-003: Mobile Experience**
> As a mobile user, I want the website to work well on my phone so that I can access it anywhere.

Acceptance Criteria:
- [x] Responsive design
- [x] Touch-friendly navigation
- [x] Fast load on mobile networks
- [x] Android app available

### Business Owner

**S-004: View Contact Submissions**
> As the business owner, I want to view contact form submissions so that I can respond to customer inquiries.

Acceptance Criteria:
- [x] Authenticated access to submissions
- [x] View all submissions with timestamps
- [ ] Export submissions (future)
- [ ] Mark as responded (future)

**S-005: Easy Deployment**
> As the site administrator, I want simple deployment so that I can update the site quickly.

Acceptance Criteria:
- [x] Single binary deployment
- [x] No external dependencies
- [x] Docker support
- [x] Systemd service configuration

### Developers

**S-006: Development Mode**
> As a developer, I want a development server that shows changes instantly so that I can iterate quickly.

Acceptance Criteria:
- [x] Dev server reads files from disk
- [x] No recompilation for HTML/CSS/JS changes
- [x] Same API as production
- [x] Clear documentation

**S-007: Performance Testing**
> As a developer, I want performance benchmarking tools so that I can measure and optimize the server.

Acceptance Criteria:
- [x] Server-side benchmark tool
- [x] Client-side performance measurement
- [x] Traceroute visualization
- [x] Documented benchmark results

---

## Roadmap

### v0.2.0 - Current Release

**Completed:**
- [x] Monolithic Rust binary with embedded assets
- [x] WebP image optimization (78% reduction)
- [x] CSS/JS minification
- [x] Lazy loading with scroll preloading
- [x] Contact form API with CSV storage
- [x] Development server (disk-based)
- [x] Desktop app wrapper
- [x] Benchmark tools
- [x] Performance documentation
- [x] Dual licensing (GPLv3 / Commercial)

### v0.3.0 - Security Hardening

**Planned:**
- [ ] Encrypted CSV storage (AES-256-GCM)
- [ ] Password hashing for admin accounts (Argon2id)
- [ ] Rate limiting on contact form
- [ ] CSRF tokens for forms
- [ ] Security audit and documentation

**User Story: Encrypted Data Storage**
> As a business owner, I want contact form data encrypted at rest so that customer information is protected if the server is compromised.

Implementation Notes:
- Use `ring` or `aes-gcm` crate for encryption
- Key stored in environment variable or secrets manager
- Encrypt each row individually for append operations
- Decrypt on read for admin view

**User Story: Secure Authentication**
> As an admin, I want my password securely hashed so that my credentials are protected.

Implementation Notes:
- Use `argon2` crate for password hashing
- Store hashed passwords in separate config file
- Implement account lockout after failed attempts

### v0.4.0 - Enhanced Monitoring

**Planned:**
- [ ] Structured logging (JSON format)
- [ ] Prometheus metrics endpoint
- [ ] Health check with detailed status
- [ ] Request tracing
- [ ] Performance regression CI

### v0.5.0 - Email Integration

**Planned:**
- [ ] SMTP email sending for contact notifications
- [ ] Email confirmation to form submitter
- [ ] Email templates (HTML + plain text)
- [ ] Configurable email settings

**User Story: Contact Notifications**
> As a business owner, I want to receive email notifications when someone submits the contact form so that I can respond promptly.

Implementation Notes:
- Use `lettre` crate for SMTP
- Queue emails in separate file (decoupled from web request)
- Background worker processes email queue
- Configurable SMTP settings via environment

### v0.6.0 - Admin Dashboard

**Planned:**
- [ ] Web-based admin interface
- [ ] Contact submission management
- [ ] Basic analytics (page views)
- [ ] Content editing (limited)

### Future Considerations

**Performance:**
- HTTP/3 QUIC support
- Edge deployment (Cloudflare Workers, Fly.io)
- Service Worker for offline support
- Image srcset for responsive images

**Features:**
- Multi-language content management
- Blog/news section
- Appointment scheduling
- Live chat integration

**Infrastructure:**
- Kubernetes deployment configs
- Auto-scaling configuration
- CDN integration
- Backup automation

---

## Technical Debt

Items to address in future releases:

1. **Test Coverage**
   - Add integration tests for all endpoints
   - Add property-based tests for validation
   - Set up CI/CD with coverage reporting

2. **Error Handling**
   - Implement custom error types
   - Better error messages for users
   - Structured error logging

3. **Configuration**
   - Environment-based configuration
   - Config file support (TOML/YAML)
   - Feature flags

4. **Documentation**
   - API documentation (OpenAPI/Swagger)
   - Architecture decision records
   - Deployment playbooks

---

## Contributing

Want to work on something from the roadmap?

1. Check if there's an open issue for the feature
2. Comment on the issue to claim it
3. Follow TDD approach (tests first)
4. Submit PR referencing the issue

For new features not on the roadmap:
1. Open an issue to discuss
2. Get approval before starting work
3. Follow existing patterns and style

---

## Version History

| Version | Date | Highlights |
|---------|------|------------|
| v0.1.0 | 2026-01-13 | Initial release, nginx + Rust API |
| v0.2.0 | 2026-01-15 | Monolithic binary, WebP, benchmarks |

---

*Last Updated: January 2026*

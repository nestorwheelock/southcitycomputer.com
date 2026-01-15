#!/bin/bash
# Production Server Setup for Ubuntu
# South City Computer - scc-server
#
# This script sets up a production server with:
# - nginx as reverse proxy
# - Let's Encrypt SSL certificates
# - systemd service
# - Firewall configuration
# - Security hardening
#
# Tested on: Ubuntu 22.04 LTS, Ubuntu 24.04 LTS
#
# Usage:
#   sudo ./scripts/setup-server-ubuntu.sh --domain southcitycomputer.com --email admin@example.com
#
# Prerequisites:
# - Fresh Ubuntu server with root access
# - Domain pointing to server's IP
# - Port 80 and 443 accessible

set -e

# =============================================================================
# Configuration
# =============================================================================

DOMAIN=""
EMAIL=""
APP_USER="scc"
APP_DIR="/opt/scc"
BINARY_NAME="scc-server"
SERVICE_NAME="southcitycomputer"
APP_PORT=9000

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[OK]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; exit 1; }

# =============================================================================
# Argument Parsing
# =============================================================================

show_help() {
    echo "Production Server Setup for South City Computer"
    echo ""
    echo "Usage: $0 [options]"
    echo ""
    echo "Required options:"
    echo "  --domain DOMAIN    Domain name (e.g., southcitycomputer.com)"
    echo "  --email EMAIL      Email for Let's Encrypt notifications"
    echo ""
    echo "Optional:"
    echo "  --user USER        Application user (default: scc)"
    echo "  --dir DIR          Application directory (default: /opt/scc)"
    echo "  --port PORT        Application port (default: 9000)"
    echo "  --skip-ssl         Skip SSL certificate setup"
    echo "  --skip-firewall    Skip firewall configuration"
    echo "  --help             Show this help"
    echo ""
    echo "Example:"
    echo "  sudo $0 --domain example.com --email admin@example.com"
}

SKIP_SSL=false
SKIP_FIREWALL=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --domain)
            DOMAIN="$2"
            shift 2
            ;;
        --email)
            EMAIL="$2"
            shift 2
            ;;
        --user)
            APP_USER="$2"
            shift 2
            ;;
        --dir)
            APP_DIR="$2"
            shift 2
            ;;
        --port)
            APP_PORT="$2"
            shift 2
            ;;
        --skip-ssl)
            SKIP_SSL=true
            shift
            ;;
        --skip-firewall)
            SKIP_FIREWALL=true
            shift
            ;;
        --help)
            show_help
            exit 0
            ;;
        *)
            log_error "Unknown option: $1"
            ;;
    esac
done

# Validate required arguments
if [ -z "$DOMAIN" ]; then
    log_error "Missing required argument: --domain"
fi

if [ -z "$EMAIL" ] && [ "$SKIP_SSL" = false ]; then
    log_error "Missing required argument: --email (or use --skip-ssl)"
fi

# Check root
if [ "$EUID" -ne 0 ]; then
    log_error "This script must be run as root (use sudo)"
fi

# =============================================================================
# Header
# =============================================================================

echo ""
echo -e "${YELLOW}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${YELLOW}║${NC}  ${GREEN}South City Computer - Production Server Setup${NC}           ${YELLOW}║${NC}"
echo -e "${YELLOW}║${NC}  ${BLUE}Ubuntu Server${NC}                                            ${YELLOW}║${NC}"
echo -e "${YELLOW}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo "Configuration:"
echo "  Domain:      $DOMAIN"
echo "  Email:       ${EMAIL:-"(skipped)"}"
echo "  App User:    $APP_USER"
echo "  App Dir:     $APP_DIR"
echo "  App Port:    $APP_PORT"
echo "  SSL:         $([ "$SKIP_SSL" = true ] && echo "Skipped" || echo "Enabled")"
echo "  Firewall:    $([ "$SKIP_FIREWALL" = true ] && echo "Skipped" || echo "Enabled")"
echo ""

read -p "Continue with installation? (y/N) " confirm
if [ "$confirm" != "y" ] && [ "$confirm" != "Y" ]; then
    log_info "Installation cancelled"
    exit 0
fi

# =============================================================================
# System Updates
# =============================================================================

log_info "Updating system packages..."
apt-get update
apt-get upgrade -y

log_success "System updated"

# =============================================================================
# Install Dependencies
# =============================================================================

log_info "Installing dependencies..."
apt-get install -y \
    nginx \
    certbot \
    python3-certbot-nginx \
    curl \
    jq \
    fail2ban \
    ufw

log_success "Dependencies installed"

# =============================================================================
# Create Application User
# =============================================================================

log_info "Creating application user: $APP_USER"

if id "$APP_USER" &>/dev/null; then
    log_info "User $APP_USER already exists"
else
    useradd --system --shell /usr/sbin/nologin --home-dir "$APP_DIR" "$APP_USER"
    log_success "User $APP_USER created"
fi

# =============================================================================
# Create Application Directory
# =============================================================================

log_info "Creating application directory: $APP_DIR"

mkdir -p "$APP_DIR"/{data,logs}
chown -R "$APP_USER:$APP_USER" "$APP_DIR"
chmod 750 "$APP_DIR"

log_success "Application directory created"

# =============================================================================
# Configure Firewall
# =============================================================================

if [ "$SKIP_FIREWALL" = false ]; then
    log_info "Configuring firewall (UFW)..."

    # Allow SSH (important - don't lock yourself out!)
    ufw allow OpenSSH

    # Allow HTTP and HTTPS
    ufw allow 'Nginx Full'

    # Enable firewall
    ufw --force enable

    log_success "Firewall configured"
    ufw status
else
    log_warning "Firewall configuration skipped"
fi

# =============================================================================
# Configure Fail2ban
# =============================================================================

log_info "Configuring Fail2ban..."

cat > /etc/fail2ban/jail.local << 'EOF'
[DEFAULT]
bantime = 1h
findtime = 10m
maxretry = 5

[sshd]
enabled = true
port = ssh
filter = sshd
logpath = /var/log/auth.log
maxretry = 3

[nginx-http-auth]
enabled = true
filter = nginx-http-auth
port = http,https
logpath = /var/log/nginx/error.log

[nginx-limit-req]
enabled = true
filter = nginx-limit-req
port = http,https
logpath = /var/log/nginx/error.log
EOF

systemctl enable fail2ban
systemctl restart fail2ban

log_success "Fail2ban configured"

# =============================================================================
# Configure nginx
# =============================================================================

log_info "Configuring nginx..."

# Remove default site
rm -f /etc/nginx/sites-enabled/default

# Create site configuration
cat > "/etc/nginx/sites-available/$DOMAIN" << EOF
# Rate limiting zone
limit_req_zone \$binary_remote_addr zone=api:10m rate=10r/s;
limit_req_zone \$binary_remote_addr zone=general:10m rate=50r/s;

# Upstream to Rust server
upstream scc_backend {
    server 127.0.0.1:$APP_PORT;
    keepalive 32;
}

server {
    listen 80;
    listen [::]:80;
    server_name $DOMAIN www.$DOMAIN;

    # ACME challenge location (for Let's Encrypt)
    location /.well-known/acme-challenge/ {
        root /var/www/html;
    }

    # Redirect all HTTP to HTTPS
    location / {
        return 301 https://\$server_name\$request_uri;
    }
}

server {
    listen 443 ssl http2;
    listen [::]:443 ssl http2;
    server_name $DOMAIN www.$DOMAIN;

    # SSL certificates (will be configured by certbot)
    ssl_certificate /etc/ssl/certs/ssl-cert-snakeoil.pem;
    ssl_certificate_key /etc/ssl/private/ssl-cert-snakeoil.key;

    # SSL configuration
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305:DHE-RSA-AES128-GCM-SHA256:DHE-RSA-AES256-GCM-SHA384;
    ssl_prefer_server_ciphers off;
    ssl_session_timeout 1d;
    ssl_session_cache shared:SSL:50m;
    ssl_session_tickets off;

    # OCSP Stapling
    ssl_stapling on;
    ssl_stapling_verify on;
    resolver 8.8.8.8 8.8.4.4 valid=300s;
    resolver_timeout 5s;

    # Security headers
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
    add_header Referrer-Policy "strict-origin-when-cross-origin" always;
    add_header Strict-Transport-Security "max-age=63072000; includeSubDomains; preload" always;

    # Logging
    access_log /var/log/nginx/${DOMAIN}_access.log;
    error_log /var/log/nginx/${DOMAIN}_error.log;

    # API endpoints with rate limiting
    location /api/ {
        limit_req zone=api burst=20 nodelay;
        proxy_pass http://scc_backend;
        proxy_http_version 1.1;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
        proxy_set_header Connection "";
    }

    # Health check (no rate limit)
    location /health {
        proxy_pass http://scc_backend;
        proxy_http_version 1.1;
        proxy_set_header Host \$host;
        proxy_set_header Connection "";
    }

    # All other requests
    location / {
        limit_req zone=general burst=100 nodelay;
        proxy_pass http://scc_backend;
        proxy_http_version 1.1;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
        proxy_set_header Connection "";

        # Caching for static assets
        location ~* \.(webp|png|jpg|jpeg|gif|ico|css|js|woff|woff2)$ {
            proxy_pass http://scc_backend;
            proxy_http_version 1.1;
            proxy_set_header Host \$host;
            proxy_set_header Connection "";
            expires 1y;
            add_header Cache-Control "public, immutable";
        }
    }

    # Deny access to hidden files
    location ~ /\. {
        deny all;
    }
}
EOF

# Enable site
ln -sf "/etc/nginx/sites-available/$DOMAIN" /etc/nginx/sites-enabled/

# Test nginx configuration
nginx -t

log_success "nginx configured"

# =============================================================================
# Create systemd Service
# =============================================================================

log_info "Creating systemd service..."

cat > "/etc/systemd/system/$SERVICE_NAME.service" << EOF
[Unit]
Description=South City Computer Website
After=network.target
Wants=network-online.target

[Service]
Type=simple
User=$APP_USER
Group=$APP_USER
WorkingDirectory=$APP_DIR
ExecStart=$APP_DIR/$BINARY_NAME
Restart=always
RestartSec=5
Environment=PORT=$APP_PORT
Environment=RUST_LOG=info

# Security hardening
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=$APP_DIR/data $APP_DIR/logs

# Resource limits
LimitNOFILE=65535
MemoryMax=512M

[Install]
WantedBy=multi-user.target
EOF

systemctl daemon-reload
systemctl enable "$SERVICE_NAME"

log_success "systemd service created"

# =============================================================================
# SSL Certificate (Let's Encrypt)
# =============================================================================

if [ "$SKIP_SSL" = false ]; then
    log_info "Obtaining SSL certificate from Let's Encrypt..."

    # Start nginx temporarily for ACME challenge
    systemctl start nginx

    # Get certificate
    certbot --nginx \
        --non-interactive \
        --agree-tos \
        --email "$EMAIL" \
        --domains "$DOMAIN" \
        --domains "www.$DOMAIN" \
        --redirect

    # Set up auto-renewal
    systemctl enable certbot.timer
    systemctl start certbot.timer

    log_success "SSL certificate obtained and auto-renewal configured"
else
    log_warning "SSL certificate setup skipped"
    log_info "You can run manually later: certbot --nginx -d $DOMAIN"
fi

# =============================================================================
# Start Services
# =============================================================================

log_info "Starting services..."

# Restart nginx with final config
systemctl restart nginx

# Note: The application service won't start until the binary is deployed
log_warning "Application service is enabled but not started"
log_info "Deploy the binary to $APP_DIR/$BINARY_NAME and run:"
log_info "  sudo systemctl start $SERVICE_NAME"

# =============================================================================
# Create Deployment Helper Script
# =============================================================================

log_info "Creating deployment helper script..."

cat > "$APP_DIR/deploy.sh" << EOF
#!/bin/bash
# Quick deployment script for $DOMAIN
# Usage: ./deploy.sh /path/to/scc-server

set -e

if [ -z "\$1" ]; then
    echo "Usage: \$0 /path/to/scc-server"
    exit 1
fi

BINARY="\$1"

if [ ! -f "\$BINARY" ]; then
    echo "Error: Binary not found: \$BINARY"
    exit 1
fi

echo "Stopping service..."
sudo systemctl stop $SERVICE_NAME || true

echo "Copying binary..."
sudo cp "\$BINARY" "$APP_DIR/$BINARY_NAME"
sudo chown $APP_USER:$APP_USER "$APP_DIR/$BINARY_NAME"
sudo chmod 755 "$APP_DIR/$BINARY_NAME"

echo "Starting service..."
sudo systemctl start $SERVICE_NAME

echo "Checking status..."
sleep 2
sudo systemctl status $SERVICE_NAME --no-pager

echo ""
echo "Testing site..."
curl -s -o /dev/null -w "HTTP Status: %{http_code}\n" http://127.0.0.1:$APP_PORT/health

echo ""
echo "Deployment complete!"
EOF

chmod +x "$APP_DIR/deploy.sh"

log_success "Deployment helper created: $APP_DIR/deploy.sh"

# =============================================================================
# Summary
# =============================================================================

echo ""
echo -e "${GREEN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║${NC}  Server setup complete!                                    ${GREEN}║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo "Configuration Summary:"
echo "  Domain:           $DOMAIN"
echo "  Application Dir:  $APP_DIR"
echo "  Application User: $APP_USER"
echo "  Service Name:     $SERVICE_NAME"
echo "  Application Port: $APP_PORT"
echo ""
echo "Next Steps:"
echo "  1. Copy your scc-server binary to the server"
echo "  2. Deploy it:"
echo "     scp scc-server user@server:/tmp/"
echo "     ssh user@server 'sudo $APP_DIR/deploy.sh /tmp/scc-server'"
echo ""
echo "  Or from your development machine:"
echo "     ./scripts/deploy.sh deploy"
echo ""
echo "Useful Commands:"
echo "  sudo systemctl status $SERVICE_NAME   # Check service status"
echo "  sudo systemctl restart $SERVICE_NAME  # Restart service"
echo "  sudo journalctl -u $SERVICE_NAME -f   # View logs"
echo "  sudo nginx -t && sudo systemctl reload nginx  # Reload nginx"
echo ""
echo "SSL Certificate:"
if [ "$SKIP_SSL" = false ]; then
    echo "  Certificate: /etc/letsencrypt/live/$DOMAIN/"
    echo "  Auto-renewal: Enabled (certbot.timer)"
    echo "  Test renewal: sudo certbot renew --dry-run"
else
    echo "  Not configured. Run: sudo certbot --nginx -d $DOMAIN"
fi
echo ""
echo "Firewall Status:"
if [ "$SKIP_FIREWALL" = false ]; then
    ufw status | grep -E "^(22|80|443|Status)"
else
    echo "  Not configured"
fi
echo ""

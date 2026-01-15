#!/bin/bash
# Deploy script for South City Computer website (bare metal)
# Usage: ./scripts/deploy.sh [command]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

# Load configuration
source "$SCRIPT_DIR/deploy.conf"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Version management
VERSION_FILE="$PROJECT_DIR/VERSION"
get_version() {
    cat "$VERSION_FILE" 2>/dev/null || echo "0.0.0"
}

show_version_banner() {
    local ver=$(get_version)
    echo ""
    echo -e "${YELLOW}╔════════════════════════════════════════╗${NC}"
    echo -e "${YELLOW}║${NC}  ${GREEN}CURRENT VERSION: v${ver}${NC}              ${YELLOW}║${NC}"
    echo -e "${YELLOW}╚════════════════════════════════════════╝${NC}"
    echo ""
}

bump_version() {
    local current=$(get_version)
    local major minor patch
    IFS='.' read -r major minor patch <<< "$current"

    case "$1" in
        major) major=$((major + 1)); minor=0; patch=0 ;;
        minor) minor=$((minor + 1)); patch=0 ;;
        patch|*) patch=$((patch + 1)) ;;
    esac

    local new_ver="$major.$minor.$patch"
    echo "$new_ver" > "$VERSION_FILE"

    # Update version in index.html
    sed -i "s/<p class=\"version-tag\">v[0-9]*\.[0-9]*\.[0-9]*<\/p>/<p class=\"version-tag\">v$new_ver<\/p>/g" "$PROJECT_DIR/index.html"

    log_success "Version bumped: v$current -> v$new_ver"
    echo "$new_ver"
}

ssh_cmd() {
    python3 -c "
import paramiko
import sys
client = paramiko.SSHClient()
client.set_missing_host_key_policy(paramiko.AutoAddPolicy())
client.connect('108.61.224.251', username='root', password='$REMOTE_PASSWORD', timeout=30)
stdin, stdout, stderr = client.exec_command('$1', timeout=${2:-300})
out = stdout.read().decode()
err = stderr.read().decode()
if out: print(out)
if err: print('STDERR:', err, file=sys.stderr)
client.close()
"
}

sftp_put() {
    python3 -c "
import paramiko
client = paramiko.SSHClient()
client.set_missing_host_key_policy(paramiko.AutoAddPolicy())
client.connect('108.61.224.251', username='root', password='$REMOTE_PASSWORD', timeout=30)
sftp = client.open_sftp()
sftp.put('$1', '$2')
sftp.close()
client.close()
print('Uploaded: $1 -> $2')
"
}

sftp_get() {
    python3 -c "
import paramiko
client = paramiko.SSHClient()
client.set_missing_host_key_policy(paramiko.AutoAddPolicy())
client.connect('108.61.224.251', username='root', password='$REMOTE_PASSWORD', timeout=30)
sftp = client.open_sftp()
sftp.get('$1', '$2')
sftp.close()
client.close()
print('Downloaded: $1 -> $2')
"
}

# =============================================================================
# Commands
# =============================================================================

cmd_status() {
    log_info "Checking production status..."
    ssh_cmd "systemctl status southcitycomputer --no-pager 2>/dev/null || echo Service_not_installed_yet"
    echo ""
    log_info "Testing site response..."
    local status=$(curl -s -o /dev/null -w "%{http_code}" "http://108.61.224.251:9000/health" 2>/dev/null || echo "000")
    if [ "$status" = "200" ]; then
        log_success "Site responding (HTTP $status)"
    else
        log_warning "Site returned HTTP $status"
    fi
}

cmd_logs() {
    log_info "Viewing service logs..."
    ssh_cmd "journalctl -u southcitycomputer -n 100 --no-pager"
}

cmd_build() {
    log_info "Building optimized release binary locally..."
    cd "$PROJECT_DIR/contact-handler"
    cargo build --release
    log_success "Binary built: target/release/scc-server ($(du -h target/release/scc-server | cut -f1))"
    log_info "This binary has all assets embedded - zero disk I/O serving"
}

cmd_deploy() {
    show_version_banner
    log_info "Deploying South City Computer - Optimized Monolithic Binary..."
    log_info "All static assets embedded in binary (WebP images, minified CSS/JS)"

    # Build locally first
    cmd_build

    # Create remote directory
    log_info "Setting up remote directory..."
    ssh_cmd "mkdir -p $REMOTE_PATH/data"

    # Stop existing service before upload
    log_info "Stopping existing service..."
    ssh_cmd "systemctl stop southcitycomputer 2>/dev/null || true"

    # Upload optimized binary (contains all embedded assets)
    log_info "Uploading optimized binary (13MB with all assets embedded)..."
    sftp_put "$PROJECT_DIR/contact-handler/target/release/scc-server" "$REMOTE_PATH/scc-server"
    ssh_cmd "chmod +x $REMOTE_PATH/scc-server"

    # Upload accounts.txt if not exists on server
    ssh_cmd "test -f $REMOTE_PATH/data/accounts.txt || echo admin:changeme > $REMOTE_PATH/data/accounts.txt"

    # Upload maintenance page
    log_info "Uploading maintenance page..."
    sftp_put "$PROJECT_DIR/maintenance.html" "$REMOTE_PATH/maintenance.html"

    # Create systemd service for optimized binary
    log_info "Creating systemd service..."
    local service_content="[Unit]
Description=South City Computer Website
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/root/southcitycomputer
ExecStart=/root/southcitycomputer/scc-server
Restart=always
RestartSec=5
Environment=PORT=9000

[Install]
WantedBy=multi-user.target"
    local encoded=$(echo "$service_content" | base64 -w0)
    ssh_cmd "echo $encoded | base64 -d > /etc/systemd/system/southcitycomputer.service"

    # Reload and start service
    log_info "Starting optimized service..."
    ssh_cmd "systemctl daemon-reload && systemctl enable southcitycomputer && systemctl restart southcitycomputer"

    sleep 2
    cmd_status

    show_version_banner
    log_success "Deployment complete! v$(get_version) is now live."
}

cmd_nginx() {
    log_info "Setting up optimized nginx config for southcitycomputer.com..."

    # Upload maintenance page
    log_info "Uploading maintenance page..."
    sftp_put "$PROJECT_DIR/maintenance.html" "$REMOTE_PATH/maintenance.html"

    # Upload nginx config file
    log_info "Uploading nginx config..."
    sftp_put "$PROJECT_DIR/nginx-southcitycomputer.conf" "/etc/nginx/sites-available/southcitycomputer"

    ssh_cmd "ln -sf /etc/nginx/sites-available/southcitycomputer /etc/nginx/sites-enabled/"
    ssh_cmd "nginx -t && systemctl reload nginx"
    log_success "Nginx configured for HTTPS with compression and maintenance page"

    log_info "Setting up SSL with certbot..."
    ssh_cmd "certbot --nginx -d southcitycomputer.com -d www.southcitycomputer.com --non-interactive --agree-tos --email admin@southcitycomputer.com --redirect" 120
    log_success "SSL configured! Site available at https://southcitycomputer.com"
}

cmd_restart() {
    log_info "Restarting service..."
    ssh_cmd "systemctl restart southcitycomputer"
    sleep 2
    cmd_status
}

cmd_stop() {
    log_info "Stopping service..."
    ssh_cmd "systemctl stop southcitycomputer"
    log_success "Service stopped"
}

cmd_contacts() {
    log_info "Fetching contact submissions..."
    ssh_cmd "cat $REMOTE_PATH/data/contacts.csv 2>/dev/null || echo 'No contacts yet'"
}

cmd_backup() {
    log_info "Backing up contacts from production..."
    mkdir -p "$BACKUP_DIR"
    local timestamp=$(date +%Y%m%d_%H%M%S)
    local backup_file="$BACKUP_DIR/contacts_${timestamp}.csv"
    sftp_get "$REMOTE_PATH/data/contacts.csv" "$backup_file" 2>/dev/null && \
        log_success "Contacts backed up to: $backup_file" || \
        log_warning "No contacts file to backup yet"
}

cmd_version() {
    show_version_banner
}

cmd_bump() {
    local type="${1:-patch}"
    log_info "Bumping version ($type)..."
    bump_version "$type"
    show_version_banner
}

cmd_help() {
    echo "Usage: ./scripts/deploy.sh [command]"
    echo ""
    echo "Commands:"
    echo "  deploy      Build and deploy to production"
    echo "  build       Build release binary locally"
    echo "  bump [type] Bump version (patch|minor|major)"
    echo "  version     Show current version"
    echo "  nginx       Setup nginx config (run once)"
    echo "  status      Check production status"
    echo "  logs        View service logs"
    echo "  restart     Restart the service"
    echo "  stop        Stop the service"
    echo "  contacts    View contact submissions"
    echo "  backup      Backup contacts from production"
    echo "  help        Show this help"
    echo ""
    echo "Version workflow:"
    echo "  ./scripts/deploy.sh bump       # Bump patch (1.0.0 -> 1.0.1)"
    echo "  ./scripts/deploy.sh bump minor # Bump minor (1.0.1 -> 1.1.0)"
    echo "  ./scripts/deploy.sh bump major # Bump major (1.1.0 -> 2.0.0)"
    echo "  ./scripts/deploy.sh deploy     # Deploy with current version"
}

# =============================================================================
# Main
# =============================================================================

case "${1:-help}" in
    deploy|push)  cmd_deploy ;;
    build)        cmd_build ;;
    bump)         cmd_bump "$2" ;;
    version|ver)  cmd_version ;;
    nginx)        cmd_nginx ;;
    status)       cmd_status ;;
    logs)         cmd_logs ;;
    restart)      cmd_restart ;;
    stop)         cmd_stop ;;
    contacts)     cmd_contacts ;;
    backup)       cmd_backup ;;
    help|--help|-h) cmd_help ;;
    *)
        log_error "Unknown command: $1"
        cmd_help
        exit 1
        ;;
esac

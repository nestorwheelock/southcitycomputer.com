#!/bin/bash
# Cloudflare configuration script for South City Computer
# Usage: ./scripts/cloudflare-setup.sh [command]
#
# Commands:
#   setup     - Apply all recommended settings
#   status    - Show current settings
#   purge     - Purge entire cache
#   test      - Test API connectivity

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[OK]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# =============================================================================
# Configuration - Set these values or use environment variables
# =============================================================================

# Option 1: Set directly here
# CF_API_TOKEN="your_api_token_here"
# CF_ZONE_ID="your_zone_id_here"

# Option 2: Load from config file
CF_CONFIG="$SCRIPT_DIR/cloudflare.conf"
if [ -f "$CF_CONFIG" ]; then
    source "$CF_CONFIG"
fi

# Option 3: Use environment variables (already set)
# export CF_API_TOKEN=xxx
# export CF_ZONE_ID=xxx

# Validate configuration
check_config() {
    if [ -z "$CF_API_TOKEN" ]; then
        log_error "CF_API_TOKEN not set"
        echo ""
        echo "Set it one of these ways:"
        echo "  1. Edit scripts/cloudflare.conf"
        echo "  2. export CF_API_TOKEN=your_token"
        echo "  3. Edit this script directly"
        echo ""
        echo "Get your API token from: https://dash.cloudflare.com/profile/api-tokens"
        echo "Required permissions: Zone:Zone Settings:Edit, Zone:Cache Purge:Purge"
        exit 1
    fi

    if [ -z "$CF_ZONE_ID" ]; then
        log_error "CF_ZONE_ID not set"
        echo ""
        echo "Find your Zone ID in the Cloudflare dashboard:"
        echo "  1. Go to your domain"
        echo "  2. Look in the right sidebar under 'API'"
        echo "  3. Copy the 'Zone ID'"
        exit 1
    fi
}

# =============================================================================
# API Helper Functions
# =============================================================================

CF_API="https://api.cloudflare.com/client/v4"

cf_get() {
    curl -s -X GET "$CF_API$1" \
        -H "Authorization: Bearer $CF_API_TOKEN" \
        -H "Content-Type: application/json"
}

cf_patch() {
    curl -s -X PATCH "$CF_API$1" \
        -H "Authorization: Bearer $CF_API_TOKEN" \
        -H "Content-Type: application/json" \
        -d "$2"
}

cf_post() {
    curl -s -X POST "$CF_API$1" \
        -H "Authorization: Bearer $CF_API_TOKEN" \
        -H "Content-Type: application/json" \
        -d "$2"
}

cf_delete() {
    curl -s -X DELETE "$CF_API$1" \
        -H "Authorization: Bearer $CF_API_TOKEN" \
        -H "Content-Type: application/json" \
        -d "$2"
}

# Check if API call succeeded
check_result() {
    local result="$1"
    local setting="$2"

    if echo "$result" | grep -q '"success":true'; then
        log_success "$setting"
    else
        log_error "$setting failed"
        echo "$result" | jq -r '.errors[0].message // .messages[0].message // "Unknown error"' 2>/dev/null || echo "$result"
    fi
}

# =============================================================================
# Settings Configuration
# =============================================================================

set_ssl_settings() {
    log_info "Configuring SSL/TLS settings..."

    # SSL Mode: Full (Strict)
    result=$(cf_patch "/zones/$CF_ZONE_ID/settings/ssl" '{"value":"strict"}')
    check_result "$result" "SSL Mode: Full (Strict)"

    # Always Use HTTPS
    result=$(cf_patch "/zones/$CF_ZONE_ID/settings/always_use_https" '{"value":"on"}')
    check_result "$result" "Always Use HTTPS: On"

    # Automatic HTTPS Rewrites
    result=$(cf_patch "/zones/$CF_ZONE_ID/settings/automatic_https_rewrites" '{"value":"on"}')
    check_result "$result" "Automatic HTTPS Rewrites: On"

    # Minimum TLS Version (1.2)
    result=$(cf_patch "/zones/$CF_ZONE_ID/settings/min_tls_version" '{"value":"1.2"}')
    check_result "$result" "Minimum TLS Version: 1.2"

    # TLS 1.3
    result=$(cf_patch "/zones/$CF_ZONE_ID/settings/tls_1_3" '{"value":"on"}')
    check_result "$result" "TLS 1.3: On"
}

set_performance_settings() {
    log_info "Configuring performance settings..."

    # Brotli compression
    result=$(cf_patch "/zones/$CF_ZONE_ID/settings/brotli" '{"value":"on"}')
    check_result "$result" "Brotli Compression: On"

    # Early Hints
    result=$(cf_patch "/zones/$CF_ZONE_ID/settings/early_hints" '{"value":"on"}')
    check_result "$result" "Early Hints: On"

    # HTTP/2
    result=$(cf_patch "/zones/$CF_ZONE_ID/settings/http2" '{"value":"on"}')
    check_result "$result" "HTTP/2: On"

    # HTTP/3 (QUIC)
    result=$(cf_patch "/zones/$CF_ZONE_ID/settings/http3" '{"value":"on"}')
    check_result "$result" "HTTP/3 (QUIC): On"

    # 0-RTT Connection Resumption
    result=$(cf_patch "/zones/$CF_ZONE_ID/settings/0rtt" '{"value":"on"}')
    check_result "$result" "0-RTT Connection Resumption: On"
}

set_caching_settings() {
    log_info "Configuring caching settings..."

    # Browser Cache TTL (respect origin headers)
    result=$(cf_patch "/zones/$CF_ZONE_ID/settings/browser_cache_ttl" '{"value":0}')
    check_result "$result" "Browser Cache TTL: Respect Origin"

    # Cache Level
    result=$(cf_patch "/zones/$CF_ZONE_ID/settings/cache_level" '{"value":"aggressive"}')
    check_result "$result" "Cache Level: Aggressive"

    # Development Mode: Off (enable caching)
    result=$(cf_patch "/zones/$CF_ZONE_ID/settings/development_mode" '{"value":"off"}')
    check_result "$result" "Development Mode: Off"
}

set_security_settings() {
    log_info "Configuring security settings..."

    # Security Level
    result=$(cf_patch "/zones/$CF_ZONE_ID/settings/security_level" '{"value":"medium"}')
    check_result "$result" "Security Level: Medium"

    # Browser Integrity Check
    result=$(cf_patch "/zones/$CF_ZONE_ID/settings/browser_check" '{"value":"on"}')
    check_result "$result" "Browser Integrity Check: On"

    # Email Obfuscation
    result=$(cf_patch "/zones/$CF_ZONE_ID/settings/email_obfuscation" '{"value":"on"}')
    check_result "$result" "Email Obfuscation: On"

    # Hotlink Protection
    result=$(cf_patch "/zones/$CF_ZONE_ID/settings/hotlink_protection" '{"value":"on"}')
    check_result "$result" "Hotlink Protection: On"

    # Challenge TTL (1 day)
    result=$(cf_patch "/zones/$CF_ZONE_ID/settings/challenge_ttl" '{"value":86400}')
    check_result "$result" "Challenge TTL: 1 day"
}

create_page_rules() {
    log_info "Creating page rules for optimal caching..."

    # Note: Free plan allows 3 page rules

    # Rule 1: Cache static assets aggressively (images, CSS, JS)
    log_info "Creating rule: Cache static assets..."
    result=$(cf_post "/zones/$CF_ZONE_ID/pagerules" '{
        "targets": [{
            "target": "url",
            "constraint": {
                "operator": "matches",
                "value": "*southcitycomputer.com/*.webp"
            }
        }],
        "actions": [
            {"id": "cache_level", "value": "cache_everything"},
            {"id": "edge_cache_ttl", "value": 2678400},
            {"id": "browser_cache_ttl", "value": 2678400}
        ],
        "priority": 1,
        "status": "active"
    }')
    check_result "$result" "Page Rule: Cache *.webp (31 days)"

    # Rule 2: Cache CSS/JS
    log_info "Creating rule: Cache CSS/JS..."
    result=$(cf_post "/zones/$CF_ZONE_ID/pagerules" '{
        "targets": [{
            "target": "url",
            "constraint": {
                "operator": "matches",
                "value": "*southcitycomputer.com/*.min.css"
            }
        }],
        "actions": [
            {"id": "cache_level", "value": "cache_everything"},
            {"id": "edge_cache_ttl", "value": 2678400}
        ],
        "priority": 2,
        "status": "active"
    }')
    check_result "$result" "Page Rule: Cache *.min.css (31 days)"

    # Rule 3: Cache HTML with shorter TTL
    log_info "Creating rule: Cache HTML..."
    result=$(cf_post "/zones/$CF_ZONE_ID/pagerules" '{
        "targets": [{
            "target": "url",
            "constraint": {
                "operator": "matches",
                "value": "*southcitycomputer.com/*.html"
            }
        }],
        "actions": [
            {"id": "cache_level", "value": "cache_everything"},
            {"id": "edge_cache_ttl", "value": 3600}
        ],
        "priority": 3,
        "status": "active"
    }')
    check_result "$result" "Page Rule: Cache *.html (1 hour)"
}

# =============================================================================
# Commands
# =============================================================================

cmd_test() {
    check_config
    log_info "Testing Cloudflare API connectivity..."

    result=$(cf_get "/zones/$CF_ZONE_ID")

    if echo "$result" | grep -q '"success":true'; then
        zone_name=$(echo "$result" | jq -r '.result.name')
        zone_status=$(echo "$result" | jq -r '.result.status')
        log_success "Connected to zone: $zone_name (status: $zone_status)"
    else
        log_error "API connection failed"
        echo "$result" | jq '.errors' 2>/dev/null || echo "$result"
        exit 1
    fi
}

cmd_status() {
    check_config
    log_info "Fetching current Cloudflare settings..."
    echo ""

    # Get all settings
    result=$(cf_get "/zones/$CF_ZONE_ID/settings")

    if echo "$result" | grep -q '"success":true'; then
        echo "SSL/TLS:"
        echo "  SSL Mode:        $(echo "$result" | jq -r '.result[] | select(.id=="ssl") | .value')"
        echo "  Always HTTPS:    $(echo "$result" | jq -r '.result[] | select(.id=="always_use_https") | .value')"
        echo "  Min TLS:         $(echo "$result" | jq -r '.result[] | select(.id=="min_tls_version") | .value')"
        echo "  TLS 1.3:         $(echo "$result" | jq -r '.result[] | select(.id=="tls_1_3") | .value')"
        echo ""
        echo "Performance:"
        echo "  Brotli:          $(echo "$result" | jq -r '.result[] | select(.id=="brotli") | .value')"
        echo "  HTTP/2:          $(echo "$result" | jq -r '.result[] | select(.id=="http2") | .value')"
        echo "  HTTP/3:          $(echo "$result" | jq -r '.result[] | select(.id=="http3") | .value')"
        echo "  Early Hints:     $(echo "$result" | jq -r '.result[] | select(.id=="early_hints") | .value')"
        echo ""
        echo "Security:"
        echo "  Security Level:  $(echo "$result" | jq -r '.result[] | select(.id=="security_level") | .value')"
        echo "  Browser Check:   $(echo "$result" | jq -r '.result[] | select(.id=="browser_check") | .value')"
        echo ""
        echo "Caching:"
        echo "  Cache Level:     $(echo "$result" | jq -r '.result[] | select(.id=="cache_level") | .value')"
        echo "  Dev Mode:        $(echo "$result" | jq -r '.result[] | select(.id=="development_mode") | .value')"
    else
        log_error "Failed to fetch settings"
        echo "$result" | jq '.errors' 2>/dev/null
    fi

    echo ""
    log_info "Page Rules:"
    rules=$(cf_get "/zones/$CF_ZONE_ID/pagerules")
    echo "$rules" | jq -r '.result[] | "  \(.priority). \(.targets[0].constraint.value) -> \(.actions[0].id)=\(.actions[0].value)"' 2>/dev/null || echo "  No page rules configured"
}

cmd_setup() {
    check_config

    echo ""
    echo -e "${YELLOW}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${YELLOW}║${NC}  ${GREEN}Cloudflare Configuration for South City Computer${NC}        ${YELLOW}║${NC}"
    echo -e "${YELLOW}╚════════════════════════════════════════════════════════════╝${NC}"
    echo ""

    # Test connection first
    cmd_test
    echo ""

    # Apply all settings
    set_ssl_settings
    echo ""
    set_performance_settings
    echo ""
    set_caching_settings
    echo ""
    set_security_settings
    echo ""

    log_info "Skipping page rules (run 'pagerules' command separately if needed)"
    echo ""

    log_success "Cloudflare configuration complete!"
    echo ""
    echo "Next steps:"
    echo "  1. Verify settings: ./scripts/cloudflare-setup.sh status"
    echo "  2. Purge cache:     ./scripts/cloudflare-setup.sh purge"
    echo "  3. Test site:       curl -I https://southcitycomputer.com"
}

cmd_pagerules() {
    check_config
    log_warning "This will create page rules. Free plan allows only 3 rules."
    echo "Existing rules will NOT be deleted automatically."
    echo ""
    read -p "Continue? (y/N) " confirm
    if [ "$confirm" = "y" ] || [ "$confirm" = "Y" ]; then
        create_page_rules
    else
        log_info "Cancelled"
    fi
}

cmd_purge() {
    check_config
    log_info "Purging entire Cloudflare cache..."

    result=$(cf_post "/zones/$CF_ZONE_ID/purge_cache" '{"purge_everything":true}')
    check_result "$result" "Cache purge"

    log_success "Cache purged. New requests will hit origin."
}

cmd_help() {
    echo "Cloudflare Setup Script for South City Computer"
    echo ""
    echo "Usage: $0 <command>"
    echo ""
    echo "Commands:"
    echo "  test       Test API connectivity"
    echo "  status     Show current Cloudflare settings"
    echo "  setup      Apply all recommended settings"
    echo "  pagerules  Create caching page rules (interactive)"
    echo "  purge      Purge entire cache"
    echo "  help       Show this help"
    echo ""
    echo "Configuration:"
    echo "  Set CF_API_TOKEN and CF_ZONE_ID in one of:"
    echo "    - scripts/cloudflare.conf"
    echo "    - Environment variables"
    echo "    - Edit this script directly"
    echo ""
    echo "Get API token: https://dash.cloudflare.com/profile/api-tokens"
    echo "  Required permissions:"
    echo "    - Zone:Zone Settings:Edit"
    echo "    - Zone:Cache Purge:Purge"
    echo "    - Zone:Page Rules:Edit (for pagerules command)"
}

# =============================================================================
# Main
# =============================================================================

case "${1:-help}" in
    test)      cmd_test ;;
    status)    cmd_status ;;
    setup)     cmd_setup ;;
    pagerules) cmd_pagerules ;;
    purge)     cmd_purge ;;
    help|*)    cmd_help ;;
esac

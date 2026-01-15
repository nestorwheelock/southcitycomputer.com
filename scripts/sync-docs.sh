#!/bin/bash
# Documentation Sync Script
# South City Computer
#
# This script converts markdown documentation files to HTML pages
# and embeds them in the website. Run before building the binary
# to ensure documentation is up-to-date.
#
# Supports both English and Spanish versions.
#
# Usage:
#   ./scripts/sync-docs.sh           # Sync all docs (both languages)
#   ./scripts/sync-docs.sh README.md # Sync specific file
#   ./scripts/sync-docs.sh --check   # Check if sync needed
#   ./scripts/sync-docs.sh --lang es # Sync Spanish docs only

# Note: Not using 'set -e' as arithmetic operations ((count++)) fail when starting from 0

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
DOCS_DIR="$PROJECT_DIR/docs"

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
# Configuration: Map markdown files to HTML pages
# =============================================================================

# Format: "source.md:output.html:title:lang"
# English documents
declare -a DOC_MAPPINGS_EN=(
    "README.md:docs/readme.html:South City Computer - Project Documentation:en"
    "WHITEPAPER.md:docs/whitepaper-full.html:The Sub-Second Website - Technical White Paper:en"
    "PERFORMANCE_TESTING.md:docs/performance.html:Performance Testing & Optimization Log:en"
    "DEVELOPER.md:docs/developer.html:Developer Guide:en"
    "ROADMAP.md:docs/roadmap.html:Product Roadmap:en"
)

# Spanish documents
declare -a DOC_MAPPINGS_ES=(
    "README-es.md:docs/readme-es.html:South City Computer - Documentación del Proyecto:es"
    "WHITEPAPER-es.md:docs/whitepaper-full-es.html:El Sitio Web Sub-Segundo - White Paper Técnico:es"
    "PERFORMANCE_TESTING-es.md:docs/performance-es.html:Pruebas de Rendimiento y Optimización:es"
    "DEVELOPER-es.md:docs/developer-es.html:Guía del Desarrollador:es"
    "ROADMAP-es.md:docs/roadmap-es.html:Hoja de Ruta del Producto:es"
)

# =============================================================================
# HTML Template with Language Switcher
# =============================================================================

generate_html() {
    local title="$1"
    local content="$2"
    local source_file="$3"
    local lang="${4:-en}"

    # Determine language-specific values
    local html_lang="$lang"
    local other_lang other_lang_label current_lang_label
    local back_text docs_title source_label synced_label

    if [ "$lang" = "es" ]; then
        other_lang="en"
        other_lang_label="English"
        current_lang_label="Español"
        back_text="← Volver a Proyectos"
        docs_title="Documentación"
        source_label="Fuente"
        synced_label="Última sincronización"
    else
        other_lang="es"
        other_lang_label="Español"
        current_lang_label="English"
        back_text="← Back to Projects"
        docs_title="Documentation"
        source_label="Source"
        synced_label="Last synced"
    fi

    # Get the other language version URL (use lowercase for consistency with file mappings)
    local current_file=$(basename "$source_file" .md | tr '[:upper:]' '[:lower:]')
    local other_url=""
    if [ "$lang" = "es" ]; then
        other_url="${current_file%-es}.html"
    else
        other_url="${current_file}-es.html"
    fi

    cat << EOF
<!DOCTYPE html>
<html lang="$html_lang">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>$title | South City Computer</title>
    <meta name="description" content="$title - Technical documentation from South City Computer">
    <link rel="stylesheet" href="../css/style.min.css">
    <link rel="icon" type="image/x-icon" href="../images/favicon.ico">
    <style>
        .doc-container {
            max-width: 900px;
            margin: 0 auto;
            padding: 120px 20px 60px;
            line-height: 1.7;
        }
        .doc-header {
            margin-bottom: 2rem;
            padding-bottom: 1rem;
            border-bottom: 2px solid #1a5276;
        }
        .doc-header h1 {
            font-size: 2rem;
            color: #333;
            margin-bottom: 0.5rem;
        }
        .doc-meta {
            color: #666;
            font-size: 0.9rem;
            display: flex;
            flex-wrap: wrap;
            gap: 1rem;
            align-items: center;
        }
        .doc-meta a {
            color: #1a5276;
        }
        .lang-switcher {
            margin-left: auto;
        }
        .lang-switcher a {
            display: inline-block;
            font-size: 1.5rem;
            text-decoration: none;
            transition: transform 0.2s;
        }
        .lang-switcher a:hover {
            transform: scale(1.2);
        }
        .doc-content h1 { font-size: 1.8rem; color: #1a5276; margin-top: 2.5rem; border-bottom: 2px solid #1a5276; padding-bottom: 0.5rem; }
        .doc-content h2 { font-size: 1.5rem; color: #1a5276; margin-top: 2rem; }
        .doc-content h3 { font-size: 1.25rem; color: #2980b9; margin-top: 1.5rem; }
        .doc-content h4 { font-size: 1.1rem; color: #333; margin-top: 1.2rem; }
        .doc-content p { margin-bottom: 1rem; color: #444; }
        .doc-content ul, .doc-content ol { margin-bottom: 1rem; padding-left: 1.5rem; }
        .doc-content li { margin-bottom: 0.5rem; }
        .doc-content pre {
            background: #1e1e1e;
            color: #d4d4d4;
            padding: 1rem;
            border-radius: 8px;
            overflow-x: auto;
            margin: 1rem 0;
            font-size: 0.9rem;
        }
        .doc-content code {
            background: #f4f4f4;
            padding: 0.2rem 0.4rem;
            border-radius: 4px;
            font-size: 0.9em;
        }
        .doc-content pre code {
            background: none;
            padding: 0;
        }
        .doc-content table {
            width: 100%;
            border-collapse: collapse;
            margin: 1rem 0;
        }
        .doc-content th, .doc-content td {
            padding: 0.75rem;
            text-align: left;
            border: 1px solid #ddd;
        }
        .doc-content th {
            background: #1a5276;
            color: white;
        }
        .doc-content tr:nth-child(even) {
            background: #f8f9fa;
        }
        .doc-content blockquote {
            border-left: 4px solid #1a5276;
            padding-left: 1rem;
            margin: 1rem 0;
            color: #666;
            font-style: italic;
        }
        .doc-content a {
            color: #1a5276;
        }
        .doc-content img {
            max-width: 100%;
            height: auto;
            border-radius: 8px;
        }
        .back-link {
            display: inline-block;
            margin-top: 2rem;
            color: #1a5276;
            text-decoration: none;
        }
        .back-link:hover {
            text-decoration: underline;
        }
        .doc-nav {
            background: #f8f9fa;
            padding: 1rem;
            border-radius: 8px;
            margin-bottom: 2rem;
        }
        .doc-nav ul {
            list-style: none;
            padding: 0;
            margin: 0;
            display: flex;
            flex-wrap: wrap;
            gap: 1rem;
        }
        .doc-nav li a {
            color: #1a5276;
            text-decoration: none;
        }
        .doc-nav li a:hover {
            text-decoration: underline;
        }
    </style>
</head>
<body>
    <script src="../js/main.min.js"></script>
    <nav class="main-nav">
        <div class="nav-container">
            <a href="../$([ "$lang" = "es" ] && echo "index-es.html" || echo "")" class="nav-logo">
                <img src="../images/logo.webp" alt="South City Computer">
            </a>
            <button class="nav-toggle" aria-label="Toggle navigation">
                <span></span>
                <span></span>
                <span></span>
            </button>
            <ul class="nav-links">
                <li><a href="../$([ "$lang" = "es" ] && echo "index-es.html" || echo "")#about">$([ "$lang" = "es" ] && echo "Nosotros" || echo "About")</a></li>
                <li><a href="../$([ "$lang" = "es" ] && echo "index-es.html" || echo "")#services">$([ "$lang" = "es" ] && echo "Servicios" || echo "Services")</a></li>
                <li><a href="../paradise$([ "$lang" = "es" ] && echo "-es" || echo "").html">Paradise</a></li>
                <li><a href="../$([ "$lang" = "es" ] && echo "index-es.html" || echo "")#projects">$([ "$lang" = "es" ] && echo "Proyectos" || echo "Projects")</a></li>
                <li><a href="../$([ "$lang" = "es" ] && echo "index-es.html" || echo "")#contact">$([ "$lang" = "es" ] && echo "Contacto" || echo "Contact")</a></li>
            </ul>
        </div>
    </nav>

    <div class="doc-container">
        <div class="doc-header">
            <h1>$title</h1>
            <div class="doc-meta">
                <span>$source_label: <a href="https://github.com/nestorwheelock/southcitycomputer.com/blob/master/$source_file" target="_blank">$source_file</a></span>
                <span>$synced_label: $(date '+%Y-%m-%d %H:%M UTC')</span>
                <div class="lang-switcher">
                    <a href="$other_url" title="$([ "$lang" = "en" ] && echo "Ver en Español" || echo "View in English")" onclick="localStorage.setItem('langManuallySet','$other_lang')">$([ "$lang" = "en" ] && echo "🇲🇽" || echo "🇺🇸")</a>
                </div>
            </div>
        </div>

        <nav class="doc-nav">
            <strong>$docs_title:</strong>
            <ul>
$(if [ "$lang" = "es" ]; then
    echo '                <li><a href="readme-es.html">README</a></li>'
    echo '                <li><a href="developer-es.html">Guía Desarrollador</a></li>'
    echo '                <li><a href="performance-es.html">Rendimiento</a></li>'
    echo '                <li><a href="whitepaper-full-es.html">White Paper</a></li>'
    echo '                <li><a href="roadmap-es.html">Hoja de Ruta</a></li>'
else
    echo '                <li><a href="readme.html">README</a></li>'
    echo '                <li><a href="developer.html">Developer Guide</a></li>'
    echo '                <li><a href="performance.html">Performance</a></li>'
    echo '                <li><a href="whitepaper-full.html">White Paper</a></li>'
    echo '                <li><a href="roadmap.html">Roadmap</a></li>'
fi)
            </ul>
        </nav>

        <div class="doc-content">
$content
        </div>

        <a href="../$([ "$lang" = "es" ] && echo "index-es.html" || echo "")#projects" class="back-link">$back_text</a>
    </div>

    <footer class="footer service-footer">
        <div class="container">
            <div class="footer-bottom">
                <p>&copy; 2006&ndash;2026 South City Computer. All rights reserved.</p>
                <p>Built in Rust at South City Computer.</p>
            </div>
        </div>
    </footer>
</body>
</html>
EOF
}

# =============================================================================
# Markdown to HTML Converter
# =============================================================================

convert_markdown() {
    local input="$1"

    # Check if pandoc is available (best option)
    if command -v pandoc &> /dev/null; then
        pandoc -f markdown -t html "$input"
        return
    fi

    # Fallback: Basic markdown conversion with sed/awk
    log_warning "pandoc not found, using basic converter"

    cat "$input" | \
    # Escape HTML entities
    sed 's/&/\&amp;/g; s/</\&lt;/g; s/>/\&gt;/g' | \
    # Code blocks (\`\`\`)
    awk '
        /^```/ {
            if (in_code) {
                print "</code></pre>"
                in_code = 0
            } else {
                print "<pre><code>"
                in_code = 1
            }
            next
        }
        in_code { print; next }
        { print }
    ' | \
    # Headers
    sed -E 's/^###### (.*)$/<h6>\1<\/h6>/' | \
    sed -E 's/^##### (.*)$/<h5>\1<\/h5>/' | \
    sed -E 's/^#### (.*)$/<h4>\1<\/h4>/' | \
    sed -E 's/^### (.*)$/<h3>\1<\/h3>/' | \
    sed -E 's/^## (.*)$/<h2>\1<\/h2>/' | \
    sed -E 's/^# (.*)$/<h1>\1<\/h1>/' | \
    # Bold and italic
    sed -E 's/\*\*\*([^*]+)\*\*\*/<strong><em>\1<\/em><\/strong>/g' | \
    sed -E 's/\*\*([^*]+)\*\*/<strong>\1<\/strong>/g' | \
    sed -E 's/\*([^*]+)\*/<em>\1<\/em>/g' | \
    # Inline code
    sed -E 's/`([^`]+)`/<code>\1<\/code>/g' | \
    # Links
    sed -E 's/\[([^\]]+)\]\(([^)]+)\)/<a href="\2">\1<\/a>/g' | \
    # Horizontal rules
    sed -E 's/^---$/<hr>/' | \
    # Paragraphs (blank lines)
    awk '
        /^$/ && !in_para { print "<p>"; in_para=1; next }
        /^<(h[1-6]|pre|ul|ol|li|hr|table|blockquote)/ { if(in_para) { print "</p>"; in_para=0 } }
        { print }
        END { if(in_para) print "</p>" }
    '
}

# =============================================================================
# Sync Functions
# =============================================================================

sync_file() {
    local mapping="$1"
    local source_file="${mapping%%:*}"
    local rest="${mapping#*:}"
    local output_file="${rest%%:*}"
    rest="${rest#*:}"
    local title="${rest%%:*}"
    local lang="${rest#*:}"

    local source_path="$PROJECT_DIR/$source_file"
    local output_path="$PROJECT_DIR/$output_file"

    if [ ! -f "$source_path" ]; then
        log_warning "Source file not found: $source_file"
        return 1
    fi

    log_info "Converting: $source_file -> $output_file ($lang)"

    # Ensure output directory exists
    mkdir -p "$(dirname "$output_path")"

    # Convert markdown to HTML
    local content
    content=$(convert_markdown "$source_path")

    # Generate full HTML page
    generate_html "$title" "$content" "$source_file" "$lang" > "$output_path"

    log_success "Generated: $output_file"
}

sync_all() {
    local lang_filter="${1:-all}"

    log_info "Syncing documentation files..."

    local count=0
    local errors=0

    # English documents
    if [ "$lang_filter" = "all" ] || [ "$lang_filter" = "en" ]; then
        for mapping in "${DOC_MAPPINGS_EN[@]}"; do
            if sync_file "$mapping"; then
                ((count++))
            else
                ((errors++))
            fi
        done
    fi

    # Spanish documents
    if [ "$lang_filter" = "all" ] || [ "$lang_filter" = "es" ]; then
        for mapping in "${DOC_MAPPINGS_ES[@]}"; do
            if sync_file "$mapping"; then
                ((count++))
            else
                ((errors++))
            fi
        done
    fi

    echo ""
    log_success "Synced $count files ($errors skipped/errors)"
}

check_sync() {
    log_info "Checking if documentation sync is needed..."

    local needs_sync=false

    # Check English
    for mapping in "${DOC_MAPPINGS_EN[@]}"; do
        local source_file="${mapping%%:*}"
        local rest="${mapping#*:}"
        local output_file="${rest%%:*}"

        local source_path="$PROJECT_DIR/$source_file"
        local output_path="$PROJECT_DIR/$output_file"

        if [ ! -f "$source_path" ]; then
            continue
        fi

        if [ ! -f "$output_path" ]; then
            log_warning "$output_file does not exist"
            needs_sync=true
            continue
        fi

        if [ "$source_path" -nt "$output_path" ]; then
            log_warning "$source_file is newer than $output_file"
            needs_sync=true
        fi
    done

    # Check Spanish
    for mapping in "${DOC_MAPPINGS_ES[@]}"; do
        local source_file="${mapping%%:*}"
        local rest="${mapping#*:}"
        local output_file="${rest%%:*}"

        local source_path="$PROJECT_DIR/$source_file"
        local output_path="$PROJECT_DIR/$output_file"

        if [ ! -f "$source_path" ]; then
            continue
        fi

        if [ ! -f "$output_path" ]; then
            log_warning "$output_file does not exist"
            needs_sync=true
            continue
        fi

        if [ "$source_path" -nt "$output_path" ]; then
            log_warning "$source_file is newer than $output_file"
            needs_sync=true
        fi
    done

    if [ "$needs_sync" = true ]; then
        log_warning "Documentation sync needed. Run: ./scripts/sync-docs.sh"
        return 1
    else
        log_success "Documentation is up to date"
        return 0
    fi
}

create_index() {
    log_info "Creating documentation index pages..."

    # English index
    create_index_page "en"

    # Spanish index
    create_index_page "es"

    log_success "Created: docs/index.html and docs/index-es.html"
}

create_index_page() {
    local lang="$1"
    local suffix=""
    [ "$lang" = "es" ] && suffix="-es"

    local index_path="$PROJECT_DIR/docs/index${suffix}.html"

    if [ "$lang" = "es" ]; then
        cat > "$index_path" << 'EOFES'
<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Documentación | South City Computer</title>
    <meta name="description" content="Documentación técnica para el servidor web de alto rendimiento de South City Computer">
    <link rel="stylesheet" href="../css/style.min.css">
    <link rel="icon" type="image/x-icon" href="../images/favicon.ico">
    <style>
        .docs-index {
            max-width: 900px;
            margin: 0 auto;
            padding: 120px 20px 60px;
        }
        .docs-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 2rem;
        }
        .docs-index h1 {
            color: #1a5276;
            margin: 0;
        }
        .lang-switcher a {
            font-size: 1.5rem;
            text-decoration: none;
            transition: transform 0.2s;
        }
        .lang-switcher a:hover {
            transform: scale(1.2);
        }
        .doc-cards {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
            gap: 1.5rem;
            margin-top: 2rem;
        }
        .doc-card {
            background: white;
            border: 1px solid #e0e0e0;
            border-radius: 12px;
            padding: 1.5rem;
            transition: transform 0.2s, box-shadow 0.2s;
        }
        .doc-card:hover {
            transform: translateY(-4px);
            box-shadow: 0 8px 25px rgba(0,0,0,0.1);
        }
        .doc-card h3 {
            color: #1a5276;
            margin-bottom: 0.5rem;
        }
        .doc-card h3 a {
            text-decoration: none;
            color: inherit;
        }
        .doc-card h3 a:hover {
            color: #2980b9;
        }
        .doc-card p {
            color: #666;
            font-size: 0.95rem;
            margin-bottom: 0;
        }
        .github-link {
            display: inline-flex;
            align-items: center;
            gap: 0.5rem;
            margin-top: 2rem;
            padding: 0.75rem 1.5rem;
            background: #24292e;
            color: white;
            border-radius: 8px;
            text-decoration: none;
        }
        .github-link:hover {
            background: #1a1e21;
        }
    </style>
</head>
<body>
    <script>
    // Browser language detection - redirect English speakers to English version
    (function() {
        if (localStorage.getItem('langManuallySet')) return;
        var lang = navigator.language || navigator.userLanguage;
        if (lang && !lang.startsWith('es')) {
            localStorage.setItem('langManuallySet', 'en');
            window.location.href = 'index.html';
        }
    })();
    </script>
    <script src="../js/main.min.js"></script>
    <nav class="main-nav">
        <div class="nav-container">
            <a href="../index-es.html" class="nav-logo">
                <img src="../images/logo.webp" alt="South City Computer">
            </a>
            <button class="nav-toggle" aria-label="Toggle navigation">
                <span></span>
                <span></span>
                <span></span>
            </button>
            <ul class="nav-links">
                <li><a href="../index-es.html#about">Nosotros</a></li>
                <li><a href="../index-es.html#services">Servicios</a></li>
                <li><a href="../paradise-es.html">Paradise</a></li>
                <li><a href="../index-es.html#projects">Proyectos</a></li>
                <li><a href="../index-es.html#contact">Contacto</a></li>
            </ul>
        </div>
    </nav>

    <div class="docs-index">
        <div class="docs-header">
            <h1>Documentación</h1>
            <div class="lang-switcher">
                <a href="index.html" title="View in English" onclick="localStorage.setItem('langManuallySet','en')">🇺🇸</a>
            </div>
        </div>
        <p>Documentación técnica para el servidor web de alto rendimiento de South City Computer construido con Rust.</p>

        <div class="doc-cards">
            <div class="doc-card">
                <h3><a href="readme-es.html">README</a></h3>
                <p>Resumen del proyecto, guía de inicio rápido e instrucciones básicas de uso.</p>
            </div>

            <div class="doc-card">
                <h3><a href="developer-es.html">Guía del Desarrollador</a></h3>
                <p>Resumen de arquitectura, estructura de código y guías de contribución.</p>
            </div>

            <div class="doc-card">
                <h3><a href="performance-es.html">Pruebas de Rendimiento</a></h3>
                <p>Metodología de benchmark, resultados y registro de optimización.</p>
            </div>

            <div class="doc-card">
                <h3><a href="whitepaper-full-es.html">White Paper Técnico</a></h3>
                <p>Profundización en nuestras técnicas de optimización de rendimiento y decisiones de arquitectura.</p>
            </div>

            <div class="doc-card">
                <h3><a href="roadmap-es.html">Hoja de Ruta</a></h3>
                <p>Funcionalidades planificadas, mejoras y objetivos de desarrollo futuro.</p>
            </div>
        </div>

        <a href="https://github.com/nestorwheelock/southcitycomputer.com" class="github-link" target="_blank">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
            </svg>
            Ver en GitHub
        </a>
    </div>

    <footer class="footer service-footer">
        <div class="container">
            <div class="footer-bottom">
                <p>&copy; 2006&ndash;2026 South City Computer. Todos los derechos reservados.</p>
                <p>Construido con Rust en South City Computer.</p>
            </div>
        </div>
    </footer>
</body>
</html>
EOFES
    else
        cat > "$index_path" << 'EOFEN'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Documentation | South City Computer</title>
    <meta name="description" content="Technical documentation for South City Computer's high-performance web server">
    <link rel="stylesheet" href="../css/style.min.css">
    <link rel="icon" type="image/x-icon" href="../images/favicon.ico">
    <style>
        .docs-index {
            max-width: 900px;
            margin: 0 auto;
            padding: 120px 20px 60px;
        }
        .docs-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 2rem;
        }
        .docs-index h1 {
            color: #1a5276;
            margin: 0;
        }
        .lang-switcher a {
            font-size: 1.5rem;
            text-decoration: none;
            transition: transform 0.2s;
        }
        .lang-switcher a:hover {
            transform: scale(1.2);
        }
        .doc-cards {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
            gap: 1.5rem;
            margin-top: 2rem;
        }
        .doc-card {
            background: white;
            border: 1px solid #e0e0e0;
            border-radius: 12px;
            padding: 1.5rem;
            transition: transform 0.2s, box-shadow 0.2s;
        }
        .doc-card:hover {
            transform: translateY(-4px);
            box-shadow: 0 8px 25px rgba(0,0,0,0.1);
        }
        .doc-card h3 {
            color: #1a5276;
            margin-bottom: 0.5rem;
        }
        .doc-card h3 a {
            text-decoration: none;
            color: inherit;
        }
        .doc-card h3 a:hover {
            color: #2980b9;
        }
        .doc-card p {
            color: #666;
            font-size: 0.95rem;
            margin-bottom: 0;
        }
        .github-link {
            display: inline-flex;
            align-items: center;
            gap: 0.5rem;
            margin-top: 2rem;
            padding: 0.75rem 1.5rem;
            background: #24292e;
            color: white;
            border-radius: 8px;
            text-decoration: none;
        }
        .github-link:hover {
            background: #1a1e21;
        }
    </style>
</head>
<body>
    <script>
    // Browser language detection - redirect Spanish speakers to Spanish version
    (function() {
        if (localStorage.getItem('langManuallySet')) return;
        var lang = navigator.language || navigator.userLanguage;
        if (lang && lang.startsWith('es')) {
            localStorage.setItem('langManuallySet', 'es');
            window.location.href = 'index-es.html';
        }
    })();
    </script>
    <script src="../js/main.min.js"></script>
    <nav class="main-nav">
        <div class="nav-container">
            <a href="../" class="nav-logo">
                <img src="../images/logo.webp" alt="South City Computer">
            </a>
            <button class="nav-toggle" aria-label="Toggle navigation">
                <span></span>
                <span></span>
                <span></span>
            </button>
            <ul class="nav-links">
                <li><a href="../#about">About</a></li>
                <li><a href="../#services">Services</a></li>
                <li><a href="../paradise.html">Paradise</a></li>
                <li><a href="../#projects">Projects</a></li>
                <li><a href="../#contact">Contact</a></li>
            </ul>
        </div>
    </nav>

    <div class="docs-index">
        <div class="docs-header">
            <h1>Documentation</h1>
            <div class="lang-switcher">
                <a href="index-es.html" title="Ver en Español" onclick="localStorage.setItem('langManuallySet','es')">🇲🇽</a>
            </div>
        </div>
        <p>Technical documentation for South City Computer's high-performance Rust web server.</p>

        <div class="doc-cards">
            <div class="doc-card">
                <h3><a href="readme.html">README</a></h3>
                <p>Project overview, quick start guide, and basic usage instructions.</p>
            </div>

            <div class="doc-card">
                <h3><a href="developer.html">Developer Guide</a></h3>
                <p>Architecture overview, code structure, and contribution guidelines.</p>
            </div>

            <div class="doc-card">
                <h3><a href="performance.html">Performance Testing</a></h3>
                <p>Benchmark methodology, results, and optimization log.</p>
            </div>

            <div class="doc-card">
                <h3><a href="whitepaper-full.html">Technical White Paper</a></h3>
                <p>Deep dive into our performance optimization techniques and architecture decisions.</p>
            </div>

            <div class="doc-card">
                <h3><a href="roadmap.html">Roadmap</a></h3>
                <p>Planned features, improvements, and future development goals.</p>
            </div>
        </div>

        <a href="https://github.com/nestorwheelock/southcitycomputer.com" class="github-link" target="_blank">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
            </svg>
            View on GitHub
        </a>
    </div>

    <footer class="footer service-footer">
        <div class="container">
            <div class="footer-bottom">
                <p>&copy; 2006&ndash;2026 South City Computer. All rights reserved.</p>
                <p>Built in Rust at South City Computer.</p>
            </div>
        </div>
    </footer>
</body>
</html>
EOFEN
    fi
}

# =============================================================================
# Main
# =============================================================================

show_help() {
    echo "Documentation Sync Script"
    echo ""
    echo "Usage: $0 [option|file]"
    echo ""
    echo "Options:"
    echo "  (none)        Sync all documentation files (both languages)"
    echo "  --check       Check if sync is needed"
    echo "  --index       Create documentation index pages only"
    echo "  --lang LANG   Sync only specified language (en|es)"
    echo "  --help        Show this help"
    echo ""
    echo "Or specify a specific file:"
    echo "  $0 README.md"
    echo ""
    echo "Available English mappings:"
    for mapping in "${DOC_MAPPINGS_EN[@]}"; do
        echo "  ${mapping%%:*}"
    done
    echo ""
    echo "Available Spanish mappings:"
    for mapping in "${DOC_MAPPINGS_ES[@]}"; do
        echo "  ${mapping%%:*}"
    done
}

LANG_FILTER="all"

case "${1:-all}" in
    --check|-c)
        check_sync
        ;;
    --help|-h)
        show_help
        ;;
    --index|-i)
        create_index
        ;;
    --lang|-l)
        LANG_FILTER="${2:-all}"
        mkdir -p "$PROJECT_DIR/docs"
        sync_all "$LANG_FILTER"
        create_index
        ;;
    all)
        mkdir -p "$PROJECT_DIR/docs"
        sync_all
        create_index
        ;;
    *.md)
        # Sync specific file - check both arrays
        found=false
        for mapping in "${DOC_MAPPINGS_EN[@]}"; do
            if [[ "$mapping" == "$1:"* ]]; then
                sync_file "$mapping"
                found=true
                break
            fi
        done
        if [ "$found" = false ]; then
            for mapping in "${DOC_MAPPINGS_ES[@]}"; do
                if [[ "$mapping" == "$1:"* ]]; then
                    sync_file "$mapping"
                    found=true
                    break
                fi
            done
        fi
        if [ "$found" = false ]; then
            log_error "No mapping found for: $1"
        fi
        ;;
    *)
        log_error "Unknown option: $1"
        show_help
        ;;
esac

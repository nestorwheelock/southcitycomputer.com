// South City Computer - Development Server
// Reads files from disk in real-time for rapid development
// Also suitable for deployment when dynamic file updates are needed

use actix_cors::Cors;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, middleware, http::header};
use actix_files as fs;
use base64::Engine;
use chrono::Local;
use mime_guess::from_path;
use printpdf::*;
use serde::{Deserialize, Serialize};
use std::fs::{self as stdfs, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use uuid::Uuid;

// Static directory - can be overridden via environment variable
fn get_static_dir() -> PathBuf {
    std::env::var("STATIC_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(".."))
}

#[derive(Debug, Deserialize)]
struct ContactForm {
    name: String,
    email: String,
    phone: Option<String>,
    message: String,
}

#[derive(Debug, Deserialize)]
struct ServiceInquiry {
    service_type: String,
    name: String,
    email: String,
    phone: Option<String>,
    details: Option<String>,
    #[serde(flatten)]
    answers: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize)]
struct ApiResponse {
    success: bool,
    message: String,
}

#[derive(Debug, Serialize)]
struct ContactResponse {
    success: bool,
    message: String,
    id: Option<String>,
    view_url: Option<String>,
}

#[derive(Debug, Clone)]
struct Submission {
    id: String,
    timestamp: String,
    name: String,
    email: String,
    phone: String,
    message: String,
}

struct ServiceInquiryRecord {
    id: String,
    timestamp: String,
    service_type: String,
    name: String,
    email: String,
    phone: String,
    details: String,
    answers: serde_json::Value,
}

fn generate_short_id() -> String {
    let uuid = Uuid::new_v4();
    let hex = uuid.simple().to_string();
    hex[..8].to_string()
}

fn escape_csv_field(field: &str) -> String {
    field.replace(',', "\\,").replace('\n', " ").replace('\r', "")
}

fn unescape_csv_field(field: &str) -> String {
    field.replace("\\,", ",")
}

fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(&next) = chars.peek() {
                if next == ',' {
                    current.push(',');
                    chars.next();
                    continue;
                }
            }
            current.push(c);
        } else if c == ',' {
            fields.push(current.clone());
            current.clear();
        } else {
            current.push(c);
        }
    }
    fields.push(current);
    fields
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

async fn handle_contact(form: web::Json<ContactForm>) -> HttpResponse {
    let id = generate_short_id();
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let name = escape_csv_field(&form.name);
    let email = escape_csv_field(&form.email);
    let phone = escape_csv_field(form.phone.as_deref().unwrap_or(""));
    let message = escape_csv_field(&form.message);

    let csv_line = format!("{},{},{},{},{},{}\n", id, timestamp, name, email, phone, message);

    let csv_path = Path::new("contacts.csv");
    let file_exists = csv_path.exists();

    let result = OpenOptions::new()
        .create(true)
        .append(true)
        .open(csv_path)
        .and_then(|mut file| {
            if !file_exists {
                writeln!(file, "id,timestamp,name,email,phone,message")?;
            }
            write!(file, "{}", csv_line)
        });

    match result {
        Ok(_) => {
            println!("[DEV] Contact saved: {} - {}", id, form.email);
            HttpResponse::Ok().json(ContactResponse {
                success: true,
                message: "Contact submitted successfully".to_string(),
                id: Some(id.clone()),
                view_url: Some(format!("/view/{}", id)),
            })
        }
        Err(e) => {
            eprintln!("[DEV] Error writing to CSV: {}", e);
            HttpResponse::InternalServerError().json(ContactResponse {
                success: false,
                message: "Failed to save contact".to_string(),
                id: None,
                view_url: None,
            })
        }
    }
}

async fn handle_service_inquiry(form: web::Json<ServiceInquiry>) -> HttpResponse {
    let id = generate_short_id();
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let service_type = escape_csv_field(&form.service_type);
    let name = escape_csv_field(&form.name);
    let email = escape_csv_field(&form.email);
    let phone = escape_csv_field(form.phone.as_deref().unwrap_or(""));
    let details = escape_csv_field(form.details.as_deref().unwrap_or(""));

    let answers_json = serde_json::to_string(&form.answers).unwrap_or_else(|_| "{}".to_string());
    let answers_escaped = escape_csv_field(&answers_json);

    let csv_line = format!("{},{},{},{},{},{},{},{}\n",
        id, timestamp, service_type, name, email, phone, details, answers_escaped);

    let csv_path = Path::new("service_inquiries.csv");
    let file_exists = csv_path.exists();

    let result = OpenOptions::new()
        .create(true)
        .append(true)
        .open(csv_path)
        .and_then(|mut file| {
            if !file_exists {
                writeln!(file, "id,timestamp,service_type,name,email,phone,details,answers")?;
            }
            write!(file, "{}", csv_line)
        });

    match result {
        Ok(_) => {
            println!("[DEV] Service inquiry saved: {} - {} - {}", id, form.service_type, form.email);
            HttpResponse::Ok().json(ContactResponse {
                success: true,
                message: "Service inquiry submitted successfully".to_string(),
                id: Some(id.clone()),
                view_url: None,
            })
        }
        Err(e) => {
            eprintln!("[DEV] Error writing service inquiry to CSV: {}", e);
            HttpResponse::InternalServerError().json(ContactResponse {
                success: false,
                message: "Failed to save inquiry".to_string(),
                id: None,
                view_url: None,
            })
        }
    }
}

fn find_submission_by_id(id: &str) -> Option<Submission> {
    let csv_path = Path::new("contacts.csv");
    let content = stdfs::read_to_string(csv_path).ok()?;

    for line in content.lines().skip(1) {
        let fields = parse_csv_line(line);
        if fields.len() >= 6 && fields[0] == id {
            return Some(Submission {
                id: fields[0].clone(),
                timestamp: fields[1].clone(),
                name: unescape_csv_field(&fields[2]),
                email: unescape_csv_field(&fields[3]),
                phone: unescape_csv_field(&fields[4]),
                message: unescape_csv_field(&fields[5]),
            });
        }
    }
    None
}

fn find_service_inquiry_by_id(id: &str) -> Option<ServiceInquiryRecord> {
    let csv_path = Path::new("service_inquiries.csv");
    let content = stdfs::read_to_string(csv_path).ok()?;

    for line in content.lines().skip(1) {
        let fields = parse_csv_line(line);
        if fields.len() >= 8 && fields[0] == id {
            let answers_raw = unescape_csv_field(&fields[7]);
            let answers: serde_json::Value = serde_json::from_str(&answers_raw)
                .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));

            return Some(ServiceInquiryRecord {
                id: fields[0].clone(),
                timestamp: fields[1].clone(),
                service_type: unescape_csv_field(&fields[2]),
                name: unescape_csv_field(&fields[3]),
                email: unescape_csv_field(&fields[4]),
                phone: unescape_csv_field(&fields[5]),
                details: unescape_csv_field(&fields[6]),
                answers,
            });
        }
    }
    None
}

async fn view_submission(path: web::Path<String>) -> HttpResponse {
    let id = path.into_inner();

    if let Some(submission) = find_submission_by_id(&id) {
        return HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(generate_view_html(&submission));
    }

    if let Some(inquiry) = find_service_inquiry_by_id(&id) {
        return HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(generate_service_inquiry_view_html(&inquiry));
    }

    HttpResponse::NotFound()
        .content_type("text/html; charset=utf-8")
        .body("<h1>404 - Not Found</h1>")
}

fn generate_view_html(sub: &Submission) -> String {
    format!(
        r#"<!DOCTYPE html>
<html><head><title>Submission {}</title></head>
<body style="font-family: sans-serif; padding: 20px;">
<h1>Contact Submission</h1>
<p><strong>ID:</strong> {}</p>
<p><strong>Date:</strong> {}</p>
<p><strong>Name:</strong> {}</p>
<p><strong>Email:</strong> {}</p>
<p><strong>Phone:</strong> {}</p>
<p><strong>Message:</strong> {}</p>
<p><a href="/view/{}/pdf">Download PDF</a></p>
</body></html>"#,
        html_escape(&sub.id),
        html_escape(&sub.id),
        html_escape(&sub.timestamp),
        html_escape(&sub.name),
        html_escape(&sub.email),
        html_escape(&sub.phone),
        html_escape(&sub.message),
        html_escape(&sub.id)
    )
}

fn generate_service_inquiry_view_html(inquiry: &ServiceInquiryRecord) -> String {
    let answers_html = if let Some(obj) = inquiry.answers.as_object() {
        obj.iter()
            .filter(|(k, _)| !["service_type", "name", "email", "phone", "details"].contains(&k.as_str()))
            .map(|(k, v)| format!("<p><strong>{}:</strong> {}</p>", html_escape(k), html_escape(&v.to_string())))
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        String::new()
    };

    format!(
        r#"<!DOCTYPE html>
<html><head><title>Service Inquiry {}</title></head>
<body style="font-family: sans-serif; padding: 20px;">
<h1>Service Inquiry: {}</h1>
<p><strong>ID:</strong> {}</p>
<p><strong>Date:</strong> {}</p>
<p><strong>Name:</strong> {}</p>
<p><strong>Email:</strong> {}</p>
<p><strong>Phone:</strong> {}</p>
<p><strong>Details:</strong> {}</p>
<h3>Form Responses</h3>
{}
</body></html>"#,
        html_escape(&inquiry.id),
        html_escape(&inquiry.service_type),
        html_escape(&inquiry.id),
        html_escape(&inquiry.timestamp),
        html_escape(&inquiry.name),
        html_escape(&inquiry.email),
        html_escape(&inquiry.phone),
        html_escape(&inquiry.details),
        answers_html
    )
}

async fn download_pdf(path: web::Path<String>) -> HttpResponse {
    let id = path.into_inner();

    match find_submission_by_id(&id) {
        Some(submission) => {
            match generate_pdf(&submission) {
                Ok(pdf_bytes) => {
                    HttpResponse::Ok()
                        .content_type("application/pdf")
                        .insert_header(("Content-Disposition", format!("attachment; filename=\"submission-{}.pdf\"", submission.id)))
                        .body(pdf_bytes)
                }
                Err(e) => {
                    eprintln!("[DEV] PDF generation error: {}", e);
                    HttpResponse::InternalServerError().body("Failed to generate PDF")
                }
            }
        }
        None => HttpResponse::NotFound().body("Submission not found"),
    }
}

fn generate_pdf(sub: &Submission) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let (doc, page1, layer1) = PdfDocument::new("Contact Submission", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;

    let mut y = 270.0;
    current_layer.use_text("CONTACT SUBMISSION", 18.0, Mm(20.0), Mm(y), &font_bold);
    y -= 15.0;
    current_layer.use_text(&format!("ID: {}", sub.id), 10.0, Mm(20.0), Mm(y), &font);
    y -= 10.0;
    current_layer.use_text(&format!("Date: {}", sub.timestamp), 10.0, Mm(20.0), Mm(y), &font);
    y -= 10.0;
    current_layer.use_text(&format!("Name: {}", sub.name), 10.0, Mm(20.0), Mm(y), &font);
    y -= 10.0;
    current_layer.use_text(&format!("Email: {}", sub.email), 10.0, Mm(20.0), Mm(y), &font);
    y -= 10.0;
    current_layer.use_text(&format!("Phone: {}", sub.phone), 10.0, Mm(20.0), Mm(y), &font);
    y -= 15.0;
    current_layer.use_text("Message:", 10.0, Mm(20.0), Mm(y), &font_bold);
    y -= 8.0;

    for chunk in sub.message.chars().collect::<Vec<_>>().chunks(80) {
        let line: String = chunk.iter().collect();
        current_layer.use_text(&line, 10.0, Mm(20.0), Mm(y), &font);
        y -= 6.0;
        if y < 30.0 { break; }
    }

    let mut buffer = BufWriter::new(Vec::new());
    doc.save(&mut buffer)?;
    Ok(buffer.into_inner()?)
}

fn check_auth(req: &HttpRequest) -> bool {
    let auth_header = match req.headers().get("Authorization") {
        Some(header) => header,
        None => return false,
    };

    let auth_str = match auth_header.to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };

    if !auth_str.starts_with("Basic ") {
        return false;
    }

    let encoded = &auth_str[6..];
    let decoded = match base64::engine::general_purpose::STANDARD.decode(encoded) {
        Ok(d) => d,
        Err(_) => return false,
    };

    let credentials = match String::from_utf8(decoded) {
        Ok(s) => s,
        Err(_) => return false,
    };

    let accounts_path = Path::new("accounts.txt");
    let accounts_content = match stdfs::read_to_string(accounts_path) {
        Ok(content) => content,
        Err(_) => return false,
    };

    for line in accounts_content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if credentials == line {
            return true;
        }
    }

    false
}

async fn contact_admin(req: HttpRequest) -> HttpResponse {
    if !check_auth(&req) {
        return HttpResponse::Unauthorized()
            .insert_header(("WWW-Authenticate", "Basic realm=\"Contact Admin\""))
            .body("Unauthorized");
    }

    let contacts: Vec<Vec<String>> = stdfs::read_to_string("contacts.csv")
        .map(|c| c.lines().skip(1).map(|l| parse_csv_line(l)).collect())
        .unwrap_or_default();

    let inquiries: Vec<Vec<String>> = stdfs::read_to_string("service_inquiries.csv")
        .map(|c| c.lines().skip(1).map(|l| parse_csv_line(l)).collect())
        .unwrap_or_default();

    let html = format!(
        r#"<!DOCTYPE html>
<html><head><title>Contact Admin (DEV)</title>
<style>
body {{ font-family: sans-serif; padding: 20px; background: #1a1a1a; color: #fff; }}
table {{ width: 100%; border-collapse: collapse; margin: 20px 0; }}
th, td {{ padding: 10px; text-align: left; border: 1px solid #333; }}
th {{ background: #333; }}
a {{ color: #00bcd4; }}
.dev-badge {{ background: #e91e8c; padding: 4px 8px; border-radius: 4px; font-size: 12px; }}
</style>
</head>
<body>
<h1>Contact Admin <span class="dev-badge">DEV MODE</span></h1>
<p>Files are read from disk on each request. Changes appear immediately.</p>
<h2>Contacts ({})</h2>
<table>
<tr><th>ID</th><th>Date</th><th>Name</th><th>Email</th><th>Message</th></tr>
{}
</table>
<h2>Service Inquiries ({})</h2>
<table>
<tr><th>ID</th><th>Date</th><th>Service</th><th>Name</th><th>Email</th></tr>
{}
</table>
</body></html>"#,
        contacts.len(),
        contacts.iter().rev().take(20).map(|f| format!(
            "<tr><td><a href='/view/{}'>{}</a></td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
            f.get(0).unwrap_or(&String::new()),
            f.get(0).unwrap_or(&String::new()),
            f.get(1).unwrap_or(&String::new()),
            f.get(2).unwrap_or(&String::new()),
            f.get(3).unwrap_or(&String::new()),
            f.get(5).unwrap_or(&String::new()),
        )).collect::<Vec<_>>().join("\n"),
        inquiries.len(),
        inquiries.iter().rev().take(20).map(|f| format!(
            "<tr><td><a href='/view/{}'>{}</a></td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
            f.get(0).unwrap_or(&String::new()),
            f.get(0).unwrap_or(&String::new()),
            f.get(1).unwrap_or(&String::new()),
            f.get(2).unwrap_or(&String::new()),
            f.get(3).unwrap_or(&String::new()),
            f.get(4).unwrap_or(&String::new()),
        )).collect::<Vec<_>>().join("\n"),
    );

    HttpResponse::Ok().content_type("text/html").body(html)
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        message: "Development server running".to_string(),
    })
}

// Serve static files from disk with no caching
async fn serve_static(req: HttpRequest) -> HttpResponse {
    let path = req.path().trim_start_matches('/');
    let static_dir = get_static_dir();

    // Build full path
    let file_path = if path.is_empty() {
        static_dir.join("index.html")
    } else {
        static_dir.join(path)
    };

    // Try exact path first
    if file_path.is_file() {
        return serve_file_from_disk(&file_path);
    }

    // Try adding .html extension
    let html_path = static_dir.join(format!("{}.html", path));
    if html_path.is_file() {
        return serve_file_from_disk(&html_path);
    }

    // Try index.html in directory
    let index_path = file_path.join("index.html");
    if index_path.is_file() {
        return serve_file_from_disk(&index_path);
    }

    HttpResponse::NotFound()
        .content_type("text/html")
        .body(format!(
            "<h1>404 - Not Found</h1><p>File not found: {}</p><p>Looking in: {:?}</p>",
            path, static_dir
        ))
}

fn serve_file_from_disk(path: &Path) -> HttpResponse {
    match stdfs::read(path) {
        Ok(content) => {
            let mime = from_path(path).first_or_octet_stream();
            HttpResponse::Ok()
                .content_type(mime.as_ref())
                // No caching in dev mode - always fresh content
                .insert_header((header::CACHE_CONTROL, "no-cache, no-store, must-revalidate"))
                .insert_header((header::PRAGMA, "no-cache"))
                .insert_header((header::EXPIRES, "0"))
                .body(content)
        }
        Err(e) => {
            eprintln!("[DEV] Error reading file {:?}: {}", path, e);
            HttpResponse::InternalServerError().body(format!("Error reading file: {}", e))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "9000".to_string());
    let bind_addr = format!("0.0.0.0:{}", port);
    let static_dir = get_static_dir();

    println!();
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║     SOUTH CITY COMPUTER - Development Server                  ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║  MODE: Development (files read from disk)                     ║");
    println!("║  Changes to HTML/CSS/JS appear immediately!                   ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();
    println!("Static files directory: {:?}", static_dir);
    println!("Server starting on http://{}", bind_addr);
    println!();
    println!("Endpoints:");
    println!("  GET  /*                   - Static files (from disk, no cache)");
    println!("  POST /api/contact         - Submit contact form");
    println!("  POST /api/service-inquiry - Submit service inquiry");
    println!("  GET  /view/{{id}}           - View submission");
    println!("  GET  /view/{{id}}/pdf       - Download PDF");
    println!("  GET  /contact-admin       - Admin panel (auth required)");
    println!("  GET  /health              - Health check");
    println!();
    println!("Environment variables:");
    println!("  PORT=9000        Server port");
    println!("  STATIC_DIR=..    Static files directory");
    println!();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::new("[DEV] %a \"%r\" %s %b %Dms"))
            .route("/api/contact", web::post().to(handle_contact))
            .route("/api/service-inquiry", web::post().to(handle_service_inquiry))
            .route("/view/{id}", web::get().to(view_submission))
            .route("/view/{id}/pdf", web::get().to(download_pdf))
            .route("/contact-admin", web::get().to(contact_admin))
            .route("/health", web::get().to(health_check))
            .default_service(web::get().to(serve_static))
    })
    .bind(&bind_addr)?
    .run()
    .await
}

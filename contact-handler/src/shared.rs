// Shared code between server and desktop application
// This file is included via include!() macro

#[derive(RustEmbed)]
#[folder = "../"]
#[include = "*.html"]
#[include = "services/*.html"]
#[include = "blog/*.html"]
#[include = "admin/*.html"]
#[include = "phpmyadmin/*.html"]
#[include = "css/*.min.css"]
#[include = "js/*.min.js"]
#[include = "images/*.webp"]
#[include = "images/*.ico"]
#[include = "images/*.png"]
#[include = "images/backgrounds/*.webp"]
#[include = "audio/*.mp3"]
#[include = "app/*"]
#[exclude = "contact-handler/*"]
#[exclude = "android-app/*"]
#[exclude = "downloaded/*"]
#[exclude = "scripts/*"]
#[exclude = "*.md"]
#[exclude = "*.txt"]
#[exclude = "*.yml"]
#[exclude = "Dockerfile"]
struct Assets;

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

#[derive(Debug, Deserialize)]
struct HoneypotAttempt {
    username: String,
    password: String,
    // Fingerprint data sent from client
    source: Option<String>,           // Which honeypot page (wordpress, django, phpmyadmin)
    screen: Option<String>,           // Screen resolution
    timezone: Option<String>,         // Timezone offset
    language: Option<String>,         // Browser language
    platform: Option<String>,         // OS platform
    cookies: Option<bool>,            // Cookies enabled
    dnt: Option<bool>,                // Do Not Track
    webgl: Option<String>,            // WebGL renderer (GPU)
    canvas_hash: Option<String>,      // Canvas fingerprint hash
    touch: Option<bool>,              // Touch support
    plugins: Option<String>,          // Browser plugins count
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
        Ok(_) => HttpResponse::Ok().json(ContactResponse {
            success: true,
            message: "Contact submitted successfully".to_string(),
            id: Some(id.clone()),
            view_url: Some(format!("/view/{}", id)),
        }),
        Err(e) => {
            eprintln!("Error writing to CSV: {}", e);
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
            eprintln!("Service inquiry saved: {} - {} - {}", id, form.service_type, form.email);
            HttpResponse::Ok().json(ContactResponse {
                success: true,
                message: "Service inquiry submitted successfully".to_string(),
                id: Some(id.clone()),
                view_url: None,
            })
        }
        Err(e) => {
            eprintln!("Error writing service inquiry to CSV: {}", e);
            HttpResponse::InternalServerError().json(ContactResponse {
                success: false,
                message: "Failed to save inquiry".to_string(),
                id: None,
                view_url: None,
            })
        }
    }
}

async fn handle_honeypot(form: web::Json<HoneypotAttempt>, req: HttpRequest) -> HttpResponse {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let username = escape_csv_field(&form.username);
    let password = escape_csv_field(&form.password);

    // Get IP from request headers (check forwarded headers first)
    let ip = req.connection_info().realip_remote_addr()
        .map(|s| s.to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // Get User-Agent from request
    let user_agent = req.headers().get("User-Agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let user_agent = escape_csv_field(&user_agent);

    // Fingerprint data from client
    let source = escape_csv_field(form.source.as_deref().unwrap_or("unknown"));
    let screen = escape_csv_field(form.screen.as_deref().unwrap_or(""));
    let timezone = escape_csv_field(form.timezone.as_deref().unwrap_or(""));
    let language = escape_csv_field(form.language.as_deref().unwrap_or(""));
    let platform = escape_csv_field(form.platform.as_deref().unwrap_or(""));
    let cookies = if form.cookies.unwrap_or(false) { "yes" } else { "no" };
    let dnt = if form.dnt.unwrap_or(false) { "yes" } else { "no" };
    let webgl = escape_csv_field(form.webgl.as_deref().unwrap_or(""));
    let canvas_hash = escape_csv_field(form.canvas_hash.as_deref().unwrap_or(""));
    let touch = if form.touch.unwrap_or(false) { "yes" } else { "no" };
    let plugins = escape_csv_field(form.plugins.as_deref().unwrap_or(""));

    let csv_line = format!("{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
        timestamp, source, username, password, ip, user_agent,
        screen, timezone, language, platform, cookies, dnt, webgl, canvas_hash, touch);

    let csv_path = Path::new("honeypot_attempts.csv");
    let file_exists = csv_path.exists();

    let result = OpenOptions::new()
        .create(true)
        .append(true)
        .open(csv_path)
        .and_then(|mut file| {
            if !file_exists {
                writeln!(file, "timestamp,source,username,password,ip,user_agent,screen,timezone,language,platform,cookies,dnt,webgl,canvas_hash,touch")?;
            }
            write!(file, "{}", csv_line)
        });

    match result {
        Ok(_) => {
            eprintln!("Honeypot triggered: {} / {} from {}", form.username, form.password, ip);
            HttpResponse::Ok().json(ApiResponse {
                success: true,
                message: "Logged".to_string(),
            })
        }
        Err(e) => {
            eprintln!("Error writing honeypot attempt to CSV: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: "Failed".to_string(),
            })
        }
    }
}

fn find_submission_by_id(id: &str) -> Option<Submission> {
    let csv_path = Path::new("contacts.csv");
    let content = fs::read_to_string(csv_path).ok()?;

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
    let content = fs::read_to_string(csv_path).ok()?;

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

    // First check regular contacts
    if let Some(submission) = find_submission_by_id(&id) {
        return HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(generate_view_html(&submission));
    }

    // Then check service inquiries
    if let Some(inquiry) = find_service_inquiry_by_id(&id) {
        return HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(generate_service_inquiry_view_html(&inquiry));
    }

    // Not found
    HttpResponse::NotFound()
        .content_type("text/html; charset=utf-8")
        .body(generate_not_found_html())
}

fn generate_service_inquiry_view_html(inquiry: &ServiceInquiryRecord) -> String {
    // Format the answers nicely
    let answers_html = if let Some(obj) = inquiry.answers.as_object() {
        obj.iter()
            .filter(|(k, _)| *k != "service_type" && *k != "name" && *k != "email" && *k != "phone" && *k != "details")
            .map(|(k, v)| {
                let value = match v {
                    serde_json::Value::Array(arr) => arr.iter()
                        .filter_map(|x| x.as_str())
                        .collect::<Vec<_>>()
                        .join(", "),
                    serde_json::Value::String(s) => s.clone(),
                    _ => v.to_string(),
                };
                format!(
                    r#"<div class="info-row"><span class="label">{}</span><span class="value">{}</span></div>"#,
                    html_escape(&k.replace("_", " ")),
                    html_escape(&value)
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        String::new()
    };

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Service Inquiry - South City Computer</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, #1a5276 0%, #2980b9 50%, #00bcd4 100%);
            min-height: 100vh;
            padding: 40px 20px;
        }}
        .container {{
            max-width: 700px;
            margin: 0 auto;
        }}
        .header {{
            text-align: center;
            margin-bottom: 32px;
        }}
        .logo {{
            height: 60px;
            margin-bottom: 16px;
            filter: brightness(0) invert(1);
        }}
        h1 {{
            color: #fff;
            font-size: 1.5rem;
            font-weight: 500;
        }}
        .service-badge {{
            display: inline-block;
            background: linear-gradient(135deg, #e91e8c, #00bcd4);
            color: #fff;
            padding: 8px 16px;
            border-radius: 20px;
            font-weight: 600;
            text-transform: uppercase;
            font-size: 0.85rem;
            margin-top: 12px;
        }}
        .card {{
            background: #fff;
            border-radius: 16px;
            padding: 32px;
            box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
        }}
        .info-row {{
            display: flex;
            justify-content: space-between;
            padding: 16px 0;
            border-bottom: 1px solid #eee;
        }}
        .info-row:last-child {{
            border-bottom: none;
        }}
        .label {{
            color: #666;
            font-size: 0.9rem;
            text-transform: capitalize;
        }}
        .value {{
            color: #333;
            font-weight: 500;
            text-align: right;
            max-width: 60%;
            word-break: break-word;
        }}
        .section-title {{
            color: #1a5276;
            font-size: 1rem;
            font-weight: 600;
            margin: 24px 0 12px 0;
            padding-top: 16px;
            border-top: 2px solid #eee;
        }}
        .section-title:first-child {{
            margin-top: 0;
            padding-top: 0;
            border-top: none;
        }}
        .message-box {{
            background: #f8f9fa;
            padding: 16px;
            border-radius: 8px;
            margin-top: 16px;
            white-space: pre-wrap;
            line-height: 1.6;
        }}
        .actions {{
            display: flex;
            gap: 16px;
            margin-top: 24px;
            justify-content: center;
        }}
        .btn {{
            display: inline-flex;
            align-items: center;
            gap: 8px;
            padding: 12px 24px;
            border-radius: 8px;
            font-weight: 500;
            text-decoration: none;
            cursor: pointer;
            border: none;
            font-size: 1rem;
        }}
        .btn-primary {{
            background: linear-gradient(135deg, #1a5276, #2980b9);
            color: #fff;
        }}
        .btn-secondary {{
            background: #f0f0f0;
            color: #333;
        }}
        .timestamp {{
            text-align: center;
            color: rgba(255,255,255,0.8);
            font-size: 0.85rem;
            margin-top: 24px;
        }}
        @media print {{
            body {{ background: #fff; padding: 20px; }}
            .card {{ box-shadow: none; border: 1px solid #ddd; }}
            .actions {{ display: none; }}
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <img src="/images/logo.webp" alt="South City Computer" class="logo">
            <h1>Service Inquiry</h1>
            <span class="service-badge">{service_type}</span>
        </div>
        <div class="card">
            <h3 class="section-title">Contact Information</h3>
            <div class="info-row">
                <span class="label">Name</span>
                <span class="value">{name}</span>
            </div>
            <div class="info-row">
                <span class="label">Email</span>
                <span class="value"><a href="mailto:{email}">{email}</a></span>
            </div>
            <div class="info-row">
                <span class="label">Phone</span>
                <span class="value">{phone}</span>
            </div>

            {answers_section}

            <h3 class="section-title">Additional Details</h3>
            <div class="message-box">{details}</div>

            <div class="actions">
                <button class="btn btn-primary" onclick="window.print()">🖨️ Print</button>
                <a href="/contact-admin" class="btn btn-secondary">← Back to Admin</a>
            </div>
        </div>
        <p class="timestamp">Submitted: {timestamp} | ID: {id}</p>
    </div>
</body>
</html>"#,
        service_type = html_escape(&inquiry.service_type),
        name = html_escape(&inquiry.name),
        email = html_escape(&inquiry.email),
        phone = html_escape(&inquiry.phone),
        details = html_escape(&inquiry.details),
        answers_section = if !answers_html.is_empty() {
            format!(r#"<h3 class="section-title">Form Responses</h3>{}"#, answers_html)
        } else {
            String::new()
        },
        timestamp = html_escape(&inquiry.timestamp),
        id = html_escape(&inquiry.id)
    )
}

fn generate_view_html(sub: &Submission) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Your Submission - South City Computer</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, #e91e8c 0%, #ffeb3b 50%, #00bcd4 100%);
            min-height: 100vh;
            padding: 40px 20px;
        }}
        .container {{
            max-width: 700px;
            margin: 0 auto;
            background: #fff;
            border-radius: 16px;
            box-shadow: 0 8px 32px rgba(0,0,0,0.2);
            overflow: hidden;
        }}
        .header {{
            background: #1a1a1a;
            color: #fff;
            padding: 32px;
            text-align: center;
        }}
        .header h1 {{
            font-size: 1.5rem;
            margin-bottom: 8px;
        }}
        .header p {{
            color: #888;
            font-size: 0.875rem;
        }}
        .content {{
            padding: 32px;
        }}
        .field {{
            margin-bottom: 24px;
        }}
        .field-label {{
            font-size: 0.75rem;
            text-transform: uppercase;
            letter-spacing: 0.5px;
            color: #888;
            margin-bottom: 8px;
        }}
        .field-value {{
            font-size: 1.125rem;
            color: #333;
            line-height: 1.6;
        }}
        .message-box {{
            background: #f5f5f5;
            padding: 20px;
            border-radius: 8px;
            border-left: 4px solid #e91e8c;
        }}
        .actions {{
            display: flex;
            gap: 16px;
            padding: 24px 32px;
            background: #f9f9f9;
            border-top: 1px solid #eee;
        }}
        .btn {{
            display: inline-block;
            padding: 12px 24px;
            border-radius: 8px;
            font-weight: 600;
            text-decoration: none;
            text-align: center;
            transition: all 0.3s ease;
        }}
        .btn-primary {{
            background: linear-gradient(135deg, #e91e8c, #00bcd4);
            color: #fff;
        }}
        .btn-primary:hover {{
            transform: translateY(-2px);
            box-shadow: 0 4px 12px rgba(233, 30, 140, 0.4);
        }}
        .btn-secondary {{
            background: #eee;
            color: #333;
        }}
        .btn-secondary:hover {{
            background: #ddd;
        }}
        .ref-id {{
            font-family: monospace;
            background: #333;
            color: #00bcd4;
            padding: 4px 8px;
            border-radius: 4px;
            font-size: 0.875rem;
        }}
        @media print {{
            body {{ background: #fff; padding: 0; }}
            .container {{ box-shadow: none; }}
            .actions {{ display: none; }}
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>Submission Received</h1>
            <p>Reference ID: <span class="ref-id">{}</span></p>
        </div>
        <div class="content">
            <div class="field">
                <div class="field-label">Submitted</div>
                <div class="field-value">{}</div>
            </div>
            <div class="field">
                <div class="field-label">Name</div>
                <div class="field-value">{}</div>
            </div>
            <div class="field">
                <div class="field-label">Email</div>
                <div class="field-value">{}</div>
            </div>
            <div class="field">
                <div class="field-label">Phone</div>
                <div class="field-value">{}</div>
            </div>
            <div class="field">
                <div class="field-label">Message</div>
                <div class="field-value message-box">{}</div>
            </div>
        </div>
        <div class="actions">
            <a href="/view/{}/pdf" class="btn btn-primary">Download PDF</a>
            <a href="/" class="btn btn-secondary">Back to Site</a>
        </div>
    </div>
</body>
</html>"#,
        html_escape(&sub.id),
        html_escape(&sub.timestamp),
        html_escape(&sub.name),
        html_escape(&sub.email),
        html_escape(if sub.phone.is_empty() { "Not provided" } else { &sub.phone }),
        html_escape(&sub.message),
        html_escape(&sub.id)
    )
}

fn generate_not_found_html() -> String {
    r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Not Found - South City Computer</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: #1a1a1a;
            color: #fff;
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            text-align: center;
            padding: 20px;
        }
        h1 {
            font-size: 4rem;
            background: linear-gradient(135deg, #e91e8c, #00bcd4);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
            margin-bottom: 16px;
        }
        p {
            color: #888;
            margin-bottom: 24px;
        }
        a {
            color: #00bcd4;
            text-decoration: none;
        }
        a:hover {
            text-decoration: underline;
        }
    </style>
</head>
<body>
    <div>
        <h1>404</h1>
        <p>Submission not found or link has expired.</p>
        <a href="/">Back to South City Computer</a>
    </div>
</body>
</html>"#.to_string()
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
                    eprintln!("PDF generation error: {}", e);
                    HttpResponse::InternalServerError().body("Failed to generate PDF")
                }
            }
        }
        None => {
            HttpResponse::NotFound().body("Submission not found")
        }
    }
}

async fn download_whitepaper_pdf(path: web::Path<String>) -> HttpResponse {
    let lang = path.into_inner();
    let is_spanish = lang == "es";

    match generate_whitepaper_pdf(is_spanish) {
        Ok(pdf_bytes) => {
            let filename = if is_spanish {
                "whitepaper-sub-second-website-es.pdf"
            } else {
                "whitepaper-sub-second-website.pdf"
            };
            HttpResponse::Ok()
                .content_type("application/pdf")
                .insert_header(("Content-Disposition", format!("attachment; filename=\"{}\"", filename)))
                .body(pdf_bytes)
        }
        Err(e) => {
            eprintln!("Whitepaper PDF generation error: {}", e);
            HttpResponse::InternalServerError().body("Failed to generate PDF")
        }
    }
}

fn generate_whitepaper_pdf(spanish: bool) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let (doc, page1, layer1) = PdfDocument::new(
        if spanish { "El Sitio Web Sub-Segundo" } else { "The Sub-Second Website" },
        Mm(210.0),
        Mm(297.0),
        "Layer 1",
    );

    let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;

    // Page 1: Title and Executive Summary
    let layer = doc.get_page(page1).get_layer(layer1);
    let mut y = 270.0;

    // Header
    layer.use_text("SOUTH CITY COMPUTER", 14.0, Mm(20.0), Mm(y), &font_bold);
    y -= 6.0;
    if spanish {
        layer.use_text("Documento Técnico | Enero 2026", 9.0, Mm(20.0), Mm(y), &font);
    } else {
        layer.use_text("Technical White Paper | January 2026", 9.0, Mm(20.0), Mm(y), &font);
    }

    // Title
    y -= 25.0;
    if spanish {
        layer.use_text("El Sitio Web Sub-Segundo", 22.0, Mm(20.0), Mm(y), &font_bold);
        y -= 10.0;
        layer.use_text("Cómo Logramos Cargas de Página 46x Más Rápidas", 11.0, Mm(20.0), Mm(y), &font);
        y -= 6.0;
        layer.use_text("Usando Rust, WebP y Arquitectura Residente en Memoria", 11.0, Mm(20.0), Mm(y), &font);
    } else {
        layer.use_text("The Sub-Second Website", 22.0, Mm(20.0), Mm(y), &font_bold);
        y -= 10.0;
        layer.use_text("How We Achieved 46x Faster Page Loads Using Rust, WebP,", 11.0, Mm(20.0), Mm(y), &font);
        y -= 6.0;
        layer.use_text("and Memory-Resident Architecture", 11.0, Mm(20.0), Mm(y), &font);
    }

    // Executive Summary
    y -= 20.0;
    if spanish {
        layer.use_text("RESUMEN EJECUTIVO", 12.0, Mm(20.0), Mm(y), &font_bold);
    } else {
        layer.use_text("EXECUTIVE SUMMARY", 12.0, Mm(20.0), Mm(y), &font_bold);
    }

    y -= 10.0;
    let summary_lines = if spanish {
        vec![
            "Este documento técnico documenta la reconstrucción completa del sitio web",
            "de South City Computer, logrando un tiempo de carga de 52 milisegundos",
            "comparado con el promedio de la industria de 2.4 segundos.",
            "",
            "Resultados Clave:",
            "• Tiempo de carga: 52ms (mejora de 46x sobre el promedio)",
            "• Tamaño binario: 13MB con todos los activos embebidos",
            "• Rendimiento del servidor: 600+ req/seg en hardware básico",
            "• Cero lectura de disco durante operación",
        ]
    } else {
        vec![
            "This white paper documents the complete rebuild of the South City Computer",
            "website, achieving a 52-millisecond page load time compared to the",
            "industry average of 2.4 seconds.",
            "",
            "Key Results:",
            "• Load time: 52ms (46x improvement over average)",
            "• Binary size: 13MB with all assets embedded",
            "• Server throughput: 600+ req/sec on commodity hardware",
            "• Zero disk reads during operation",
        ]
    };

    for line in summary_lines {
        layer.use_text(line, 10.0, Mm(20.0), Mm(y), &font);
        y -= 5.5;
    }

    // Problem Statement
    y -= 12.0;
    if spanish {
        layer.use_text("EL PROBLEMA", 12.0, Mm(20.0), Mm(y), &font_bold);
    } else {
        layer.use_text("THE PROBLEM", 12.0, Mm(20.0), Mm(y), &font_bold);
    }

    y -= 10.0;
    let problem_lines = if spanish {
        vec![
            "Los sitios web modernos sufren de sobrecarga de complejidad:",
            "",
            "• El sitio promedio de WordPress tarda 2.5-3 segundos en cargar",
            "• Cada segundo de retraso reduce las conversiones en 7%",
            "• El 53% de los usuarios abandonan sitios que tardan >3 segundos",
            "• Los tiempos de carga en móvil son aún peores (13+ segundos)",
        ]
    } else {
        vec![
            "Modern websites suffer from complexity bloat:",
            "",
            "• Average WordPress site takes 2.5-3 seconds to load",
            "• Each second of delay reduces conversions by 7%",
            "• 53% of users abandon sites that take >3 seconds",
            "• Mobile load times are even worse (13+ seconds)",
        ]
    };

    for line in problem_lines {
        layer.use_text(line, 10.0, Mm(20.0), Mm(y), &font);
        y -= 5.5;
    }

    // Technical Architecture section
    y -= 12.0;
    if spanish {
        layer.use_text("ARQUITECTURA TÉCNICA", 12.0, Mm(20.0), Mm(y), &font_bold);
    } else {
        layer.use_text("TECHNICAL ARCHITECTURE", 12.0, Mm(20.0), Mm(y), &font_bold);
    }

    y -= 10.0;
    let arch_lines = if spanish {
        vec![
            "Nuestra solución utiliza una arquitectura monolítica de binario único:",
            "",
            "1. Rust + Actix Web: Servidor web de alto rendimiento",
            "2. rust-embed: Todos los activos compilados en el binario",
            "3. Imágenes WebP: 78% de reducción en tamaño de imágenes",
            "4. CSS/JS minificado: Recursos optimizados",
            "5. Sin base de datos: Sin latencia de consultas",
            "",
            "Esto elimina toda E/S de disco durante operación normal,",
            "sirviendo todo el contenido directamente desde memoria.",
        ]
    } else {
        vec![
            "Our solution uses a single-binary monolithic architecture:",
            "",
            "1. Rust + Actix Web: High-performance web server",
            "2. rust-embed: All assets compiled into the binary",
            "3. WebP images: 78% reduction in image size",
            "4. Minified CSS/JS: Optimized resources",
            "5. No database: Zero query latency",
            "",
            "This eliminates all disk I/O during normal operation,",
            "serving all content directly from memory.",
        ]
    };

    for line in arch_lines {
        layer.use_text(line, 10.0, Mm(20.0), Mm(y), &font);
        y -= 5.5;
    }

    // Page 2: Performance Data
    let (page2, layer2) = doc.add_page(Mm(210.0), Mm(297.0), "Layer 2");
    let layer = doc.get_page(page2).get_layer(layer2);
    y = 270.0;

    if spanish {
        layer.use_text("DATOS DE RENDIMIENTO", 12.0, Mm(20.0), Mm(y), &font_bold);
    } else {
        layer.use_text("PERFORMANCE DATA", 12.0, Mm(20.0), Mm(y), &font_bold);
    }

    y -= 12.0;
    if spanish {
        layer.use_text("Métricas de Prueba de Carga (1000 solicitudes):", 10.0, Mm(20.0), Mm(y), &font_bold);
    } else {
        layer.use_text("Load Test Metrics (1000 requests):", 10.0, Mm(20.0), Mm(y), &font_bold);
    }

    y -= 8.0;
    let perf_lines = if spanish {
        vec![
            "• Tiempo promedio de respuesta: 52ms",
            "• Percentil 95: 68ms",
            "• Rendimiento: 612 solicitudes/segundo",
            "• Cero errores en todas las pruebas",
            "",
            "Comparación con Sitios Web Típicos:",
            "",
            "  Nuestra Solución:     52ms   ████",
            "  Sitio estático:       200ms  ████████████████",
            "  WordPress optimizado: 800ms  ██████████████████████████████████████████████████",
            "  WordPress promedio:   2400ms [barra truncada - 46x más largo]",
        ]
    } else {
        vec![
            "• Average response time: 52ms",
            "• 95th percentile: 68ms",
            "• Throughput: 612 requests/second",
            "• Zero errors across all tests",
            "",
            "Comparison with Typical Websites:",
            "",
            "  Our Solution:         52ms   ████",
            "  Static site:          200ms  ████████████████",
            "  Optimized WordPress:  800ms  ██████████████████████████████████████████████████",
            "  Average WordPress:    2400ms [bar truncated - 46x longer]",
        ]
    };

    for line in perf_lines {
        layer.use_text(line, 10.0, Mm(20.0), Mm(y), &font);
        y -= 5.5;
    }

    // Image Optimization
    y -= 12.0;
    if spanish {
        layer.use_text("OPTIMIZACIÓN DE IMÁGENES", 12.0, Mm(20.0), Mm(y), &font_bold);
    } else {
        layer.use_text("IMAGE OPTIMIZATION", 12.0, Mm(20.0), Mm(y), &font_bold);
    }

    y -= 10.0;
    let img_lines = if spanish {
        vec![
            "Conversión de PNG/JPEG a WebP:",
            "",
            "  Imagen              Original    WebP     Reducción",
            "  ──────────────────────────────────────────────────",
            "  Storefront          3.2MB       687KB    78.5%",
            "  Wall mural          2.1MB       134KB    93.6%",
            "  Store interior      1.8MB       245KB    86.4%",
            "  ──────────────────────────────────────────────────",
            "  TOTAL               20MB        4.3MB    78.5%",
        ]
    } else {
        vec![
            "PNG/JPEG to WebP Conversion:",
            "",
            "  Image               Original    WebP     Reduction",
            "  ──────────────────────────────────────────────────",
            "  Storefront          3.2MB       687KB    78.5%",
            "  Wall mural          2.1MB       134KB    93.6%",
            "  Store interior      1.8MB       245KB    86.4%",
            "  ──────────────────────────────────────────────────",
            "  TOTAL               20MB        4.3MB    78.5%",
        ]
    };

    for line in img_lines {
        layer.use_text(line, 9.0, Mm(20.0), Mm(y), &font);
        y -= 5.0;
    }

    // Conclusion
    y -= 15.0;
    if spanish {
        layer.use_text("CONCLUSIÓN", 12.0, Mm(20.0), Mm(y), &font_bold);
    } else {
        layer.use_text("CONCLUSION", 12.0, Mm(20.0), Mm(y), &font_bold);
    }

    y -= 10.0;
    let conclusion_lines = if spanish {
        vec![
            "Al eliminar la complejidad innecesaria y enfocarnos en los fundamentos,",
            "logramos mejoras dramáticas de rendimiento:",
            "",
            "• 46x más rápido que el sitio web promedio",
            "• 20x menos solicitudes de red",
            "• Cero dependencias de base de datos",
            "• Binario único y fácil de desplegar",
            "",
            "Estos principios pueden aplicarse a cualquier sitio web, desde simples",
            "páginas de negocio hasta aplicaciones empresariales complejas.",
        ]
    } else {
        vec![
            "By eliminating unnecessary complexity and focusing on fundamentals,",
            "we achieved dramatic performance improvements:",
            "",
            "• 46x faster than average website",
            "• 20x fewer network requests",
            "• Zero database dependencies",
            "• Single binary, easy to deploy",
            "",
            "These principles can be applied to any website, from simple business",
            "pages to complex enterprise applications.",
        ]
    };

    for line in conclusion_lines {
        layer.use_text(line, 10.0, Mm(20.0), Mm(y), &font);
        y -= 5.5;
    }

    // Contact info
    y -= 15.0;
    if spanish {
        layer.use_text("CONTACTO", 10.0, Mm(20.0), Mm(y), &font_bold);
    } else {
        layer.use_text("CONTACT", 10.0, Mm(20.0), Mm(y), &font_bold);
    }
    y -= 7.0;
    layer.use_text("South City Computer", 10.0, Mm(20.0), Mm(y), &font);
    y -= 5.0;
    layer.use_text("Puerto Morelos, Mexico", 10.0, Mm(20.0), Mm(y), &font);
    y -= 5.0;
    layer.use_text("www.southcitycomputer.com", 10.0, Mm(20.0), Mm(y), &font);

    // Footer on both pages
    let footer_text = if spanish {
        "South City Computer | Puerto Morelos, Mexico | southcitycomputer.com"
    } else {
        "South City Computer | Puerto Morelos, Mexico | southcitycomputer.com"
    };

    let layer1 = doc.get_page(page1).get_layer(layer1);
    layer1.use_text(footer_text, 8.0, Mm(20.0), Mm(10.0), &font);

    let layer2 = doc.get_page(page2).get_layer(layer2);
    layer2.use_text(footer_text, 8.0, Mm(20.0), Mm(10.0), &font);

    let mut buffer = BufWriter::new(Vec::new());
    doc.save(&mut buffer)?;
    Ok(buffer.into_inner()?)
}

fn generate_pdf(sub: &Submission) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let (doc, page1, layer1) = PdfDocument::new(
        "Contact Submission",
        Mm(210.0),
        Mm(297.0),
        "Layer 1",
    );

    let current_layer = doc.get_page(page1).get_layer(layer1);
    let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;

    let mut y_pos = 270.0;

    current_layer.use_text("SOUTH CITY COMPUTER", 18.0, Mm(20.0), Mm(y_pos), &font_bold);
    y_pos -= 8.0;
    current_layer.use_text("Contact Form Submission", 12.0, Mm(20.0), Mm(y_pos), &font);

    y_pos -= 20.0;
    current_layer.use_text(&format!("Reference ID: {}", sub.id), 10.0, Mm(20.0), Mm(y_pos), &font);

    y_pos -= 20.0;
    current_layer.use_text("SUBMITTED", 8.0, Mm(20.0), Mm(y_pos), &font_bold);
    y_pos -= 6.0;
    current_layer.use_text(&sub.timestamp, 11.0, Mm(20.0), Mm(y_pos), &font);

    y_pos -= 15.0;
    current_layer.use_text("NAME", 8.0, Mm(20.0), Mm(y_pos), &font_bold);
    y_pos -= 6.0;
    current_layer.use_text(&sub.name, 11.0, Mm(20.0), Mm(y_pos), &font);

    y_pos -= 15.0;
    current_layer.use_text("EMAIL", 8.0, Mm(20.0), Mm(y_pos), &font_bold);
    y_pos -= 6.0;
    current_layer.use_text(&sub.email, 11.0, Mm(20.0), Mm(y_pos), &font);

    y_pos -= 15.0;
    current_layer.use_text("PHONE", 8.0, Mm(20.0), Mm(y_pos), &font_bold);
    y_pos -= 6.0;
    let phone_text = if sub.phone.is_empty() { "Not provided" } else { &sub.phone };
    current_layer.use_text(phone_text, 11.0, Mm(20.0), Mm(y_pos), &font);

    y_pos -= 15.0;
    current_layer.use_text("MESSAGE", 8.0, Mm(20.0), Mm(y_pos), &font_bold);
    y_pos -= 6.0;

    let max_chars_per_line = 80;
    for line in sub.message.chars().collect::<Vec<_>>().chunks(max_chars_per_line) {
        let line_text: String = line.iter().collect();
        current_layer.use_text(&line_text, 10.0, Mm(20.0), Mm(y_pos), &font);
        y_pos -= 5.0;
        if y_pos < 30.0 {
            break;
        }
    }

    y_pos = 15.0;
    current_layer.use_text("South City Computer | Puerto Morelos, Mexico | southcitycomputer.com", 8.0, Mm(20.0), Mm(y_pos), &font);

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
    let accounts_content = match fs::read_to_string(accounts_path) {
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

async fn contact_admin(req: HttpRequest) -> HttpResponse {
    if !check_auth(&req) {
        return HttpResponse::Unauthorized()
            .insert_header(("WWW-Authenticate", "Basic realm=\"Contact Admin\""))
            .body("Unauthorized");
    }

    // Read regular contacts
    let contacts: Vec<Vec<String>> = match fs::read_to_string("contacts.csv") {
        Ok(content) => content.lines().skip(1).map(|line| parse_csv_line(line)).collect(),
        Err(_) => Vec::new(),
    };

    // Read service inquiries
    let service_inquiries: Vec<Vec<String>> = match fs::read_to_string("service_inquiries.csv") {
        Ok(content) => content.lines().skip(1).map(|line| parse_csv_line(line)).collect(),
        Err(_) => Vec::new(),
    };

    // Read honeypot attempts
    let honeypot_attempts: Vec<Vec<String>> = match fs::read_to_string("honeypot_attempts.csv") {
        Ok(content) => content.lines().skip(1).map(|line| parse_csv_line(line)).collect(),
        Err(_) => Vec::new(),
    };

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(generate_admin_html(&contacts, &service_inquiries, &honeypot_attempts))
}

fn generate_admin_html(contacts: &[Vec<String>], service_inquiries: &[Vec<String>], honeypot_attempts: &[Vec<String>]) -> String {
    let contact_rows = if contacts.is_empty() {
        "<tr><td colspan=\"6\" style=\"text-align: center; padding: 40px; color: #888;\">No contacts yet</td></tr>".to_string()
    } else {
        contacts
            .iter()
            .rev()
            .map(|fields| {
                let id = fields.get(0).map(|s| html_escape(s)).unwrap_or_default();
                let timestamp = fields.get(1).map(|s| html_escape(s)).unwrap_or_default();
                let name = fields.get(2).map(|s| html_escape(&unescape_csv_field(s))).unwrap_or_default();
                let email = fields.get(3).map(|s| html_escape(&unescape_csv_field(s))).unwrap_or_default();
                let phone = fields.get(4).map(|s| html_escape(&unescape_csv_field(s))).unwrap_or_default();
                let message = fields.get(5).map(|s| html_escape(&unescape_csv_field(s))).unwrap_or_default();

                format!(
                    "<tr><td><a href=\"/view/{}\">{}</a></td><td>{}</td><td>{}</td><td><a href=\"mailto:{}\">{}</a></td><td>{}</td><td>{}</td></tr>",
                    id, id, timestamp, name, email, email, phone, message
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    let service_rows = if service_inquiries.is_empty() {
        "<tr><td colspan=\"9\" style=\"text-align: center; padding: 40px; color: #888;\">No service inquiries yet</td></tr>".to_string()
    } else {
        service_inquiries
            .iter()
            .rev()
            .map(|fields| {
                let id = fields.get(0).map(|s| html_escape(s)).unwrap_or_default();
                let timestamp = fields.get(1).map(|s| html_escape(s)).unwrap_or_default();
                let service_type = fields.get(2).map(|s| html_escape(&unescape_csv_field(s))).unwrap_or_default();
                let name = fields.get(3).map(|s| html_escape(&unescape_csv_field(s))).unwrap_or_default();
                let email = fields.get(4).map(|s| html_escape(&unescape_csv_field(s))).unwrap_or_default();
                let phone = fields.get(5).map(|s| html_escape(&unescape_csv_field(s))).unwrap_or_default();
                let details = fields.get(6).map(|s| html_escape(&unescape_csv_field(s))).unwrap_or_default();
                let answers_raw = fields.get(7).map(|s| unescape_csv_field(s)).unwrap_or_default();

                // Parse JSON answers and format nicely
                let answers_html = if let Ok(answers) = serde_json::from_str::<serde_json::Value>(&answers_raw) {
                    if let Some(obj) = answers.as_object() {
                        obj.iter()
                            .filter(|(k, _)| *k != "service_type" && *k != "name" && *k != "email" && *k != "phone" && *k != "details")
                            .map(|(k, v)| {
                                let value = match v {
                                    serde_json::Value::Array(arr) => arr.iter()
                                        .filter_map(|x| x.as_str())
                                        .collect::<Vec<_>>()
                                        .join(", "),
                                    serde_json::Value::String(s) => s.clone(),
                                    _ => v.to_string(),
                                };
                                format!("<div class=\"answer-item\"><span class=\"answer-key\">{}</span>: {}</div>",
                                    html_escape(k), html_escape(&value))
                            })
                            .collect::<Vec<_>>()
                            .join("")
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                };

                format!(
                    "<tr><td><a href=\"/view/{}\">{}</a></td><td>{}</td><td><span class=\"service-tag\">{}</span></td><td>{}</td><td><a href=\"mailto:{}\">{}</a></td><td>{}</td><td>{}</td><td class=\"answers-cell\">{}</td></tr>",
                    id, id, timestamp, service_type, name, email, email, phone, details, answers_html
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    // Collect unique IPs for map (with timestamp for JS)
    let honeypot_ips: Vec<String> = honeypot_attempts
        .iter()
        .filter_map(|fields| {
            let ip = fields.get(4)?;
            if ip.is_empty() || ip == "unknown" || ip == "127.0.0.1" { return None; }
            let timestamp = fields.get(0).unwrap_or(&String::new()).clone();
            let source = fields.get(1).unwrap_or(&String::new()).clone();
            Some(format!("{{\"ip\":\"{}\",\"time\":\"{}\",\"source\":\"{}\"}}", ip, timestamp, source))
        })
        .collect();
    let honeypot_ips_json = format!("[{}]", honeypot_ips.join(","));

    let honeypot_rows = if honeypot_attempts.is_empty() {
        "<tr><td colspan=\"8\" style=\"text-align: center; padding: 40px; color: #888;\">No honeypot attempts yet</td></tr>".to_string()
    } else {
        honeypot_attempts
            .iter()
            .rev()
            .map(|fields| {
                // New CSV format: timestamp,source,username,password,ip,user_agent,screen,timezone,language,platform,cookies,dnt,webgl,canvas_hash,touch
                let timestamp = fields.get(0).map(|s| html_escape(s)).unwrap_or_default();
                let source = fields.get(1).map(|s| html_escape(&unescape_csv_field(s))).unwrap_or_default();
                let username = fields.get(2).map(|s| html_escape(&unescape_csv_field(s))).unwrap_or_default();
                let password = fields.get(3).map(|s| html_escape(&unescape_csv_field(s))).unwrap_or_default();
                let ip = fields.get(4).map(|s| html_escape(s)).unwrap_or_default();
                let screen = fields.get(6).map(|s| html_escape(s)).unwrap_or_default();
                let platform = fields.get(9).map(|s| html_escape(s)).unwrap_or_default();
                let webgl = fields.get(12).map(|s| html_escape(&unescape_csv_field(s))).unwrap_or_default();

                format!(
                    "<tr><td>{}</td><td class=\"source-badge\">{}</td><td class=\"honeypot-cred\">{}</td><td class=\"honeypot-cred\">{}</td><td class=\"ip-cell\" data-ip=\"{}\">{}</td><td>{}</td><td>{}</td><td class=\"webgl-cell\">{}</td></tr>",
                    timestamp, source, username, password, ip, ip, screen, platform, webgl
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Contact Admin - South City Computer</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: #1a1a1a;
            color: #fff;
            min-height: 100vh;
            padding: 20px;
        }}
        .container {{
            max-width: 1400px;
            margin: 0 auto;
        }}
        h1, h2 {{
            margin-bottom: 24px;
            background: linear-gradient(135deg, #e91e8c, #00bcd4);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
        }}
        h2 {{
            margin-top: 48px;
        }}
        .stats {{
            display: flex;
            gap: 16px;
            margin-bottom: 24px;
            flex-wrap: wrap;
        }}
        .stat {{
            background: #333;
            padding: 16px 24px;
            border-radius: 8px;
        }}
        .stat-value {{
            font-size: 2rem;
            font-weight: bold;
            color: #00bcd4;
        }}
        .stat.magenta .stat-value {{
            color: #e91e8c;
        }}
        .stat-label {{
            color: #888;
            font-size: 0.875rem;
        }}
        table {{
            width: 100%;
            border-collapse: collapse;
            background: #333;
            border-radius: 8px;
            overflow: hidden;
            margin-bottom: 24px;
        }}
        th, td {{
            padding: 16px;
            text-align: left;
            border-bottom: 1px solid #444;
        }}
        th {{
            background: linear-gradient(135deg, #e91e8c, #00bcd4);
            font-weight: 600;
            text-transform: uppercase;
            font-size: 0.75rem;
            letter-spacing: 0.5px;
        }}
        tr:hover {{
            background: #3a3a3a;
        }}
        td:last-child {{
            max-width: 300px;
            word-wrap: break-word;
        }}
        a {{
            color: #00bcd4;
            text-decoration: none;
        }}
        a:hover {{
            text-decoration: underline;
        }}
        .back-link {{
            display: inline-block;
            margin-bottom: 24px;
            color: #888;
        }}
        .back-link:hover {{
            color: #00bcd4;
        }}
        .service-tag {{
            background: linear-gradient(135deg, #e91e8c, #00bcd4);
            padding: 4px 10px;
            border-radius: 12px;
            font-size: 0.75rem;
            font-weight: 600;
            text-transform: uppercase;
        }}
        .answers-cell {{
            max-width: 350px;
            font-size: 0.85rem;
        }}
        .answer-item {{
            padding: 4px 0;
            border-bottom: 1px solid #444;
        }}
        .answer-item:last-child {{
            border-bottom: none;
        }}
        .answer-key {{
            color: #00bcd4;
            font-weight: 600;
            text-transform: capitalize;
        }}
        .honeypot-cred {{
            font-family: monospace;
            background: #2a2a2a;
            padding: 4px 8px;
            border-radius: 4px;
            color: #ff6b6b;
        }}
        .source-badge {{
            font-size: 0.7rem;
            padding: 4px 8px;
            border-radius: 4px;
            background: #8b5cf6;
            color: #fff;
            text-transform: uppercase;
            font-weight: 600;
        }}
        .ip-cell {{
            font-family: monospace;
            color: #f59e0b;
        }}
        .webgl-cell {{
            max-width: 200px;
            font-size: 0.7rem;
            color: #888;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }}
        .ua-cell {{
            max-width: 300px;
            font-size: 0.75rem;
            color: #888;
            word-break: break-all;
        }}
        .stat.red .stat-value {{
            color: #ff6b6b;
        }}
    </style>
</head>
<body>
    <div class="container">
        <a href="/" class="back-link">&larr; Back to site</a>
        <h1>Contact Admin</h1>
        <div class="stats">
            <div class="stat">
                <div class="stat-value">{}</div>
                <div class="stat-label">Contact Form</div>
            </div>
            <div class="stat magenta">
                <div class="stat-value">{}</div>
                <div class="stat-label">Service Inquiries</div>
            </div>
            <div class="stat red">
                <div class="stat-value">{}</div>
                <div class="stat-label">Honeypot Catches</div>
            </div>
            <div class="stat">
                <div class="stat-value">{}</div>
                <div class="stat-label">Total Legitimate</div>
            </div>
        </div>

        <h2>Contact Form Submissions</h2>
        <table>
            <thead>
                <tr>
                    <th>ID</th>
                    <th>Timestamp</th>
                    <th>Name</th>
                    <th>Email</th>
                    <th>Phone</th>
                    <th>Message</th>
                </tr>
            </thead>
            <tbody>
                {}
            </tbody>
        </table>

        <h2>Service Inquiries</h2>
        <table>
            <thead>
                <tr>
                    <th>ID</th>
                    <th>Timestamp</th>
                    <th>Service</th>
                    <th>Name</th>
                    <th>Email</th>
                    <th>Phone</th>
                    <th>Details</th>
                    <th>Answers</th>
                </tr>
            </thead>
            <tbody>
                {}
            </tbody>
        </table>

        <h2>Honeypot Attack Map</h2>
        <div id="attack-map" style="height: 400px; border-radius: 12px; margin-bottom: 30px; background: #2a2a2a;"></div>

        <h2>Honeypot Catches</h2>
        <table>
            <thead>
                <tr>
                    <th>Timestamp</th>
                    <th>Source</th>
                    <th>Username</th>
                    <th>Password</th>
                    <th>IP</th>
                    <th>Screen</th>
                    <th>Platform</th>
                    <th>GPU</th>
                </tr>
            </thead>
            <tbody>
                {}
            </tbody>
        </table>
    </div>

    <link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" />
    <script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"></script>
    <script>
    (function() {{
        var attempts = {};
        if (attempts.length === 0) {{
            document.getElementById('attack-map').innerHTML = '<div style="display:flex;align-items:center;justify-content:center;height:100%;color:#888;">No attack data with valid IPs yet</div>';
            return;
        }}

        var map = L.map('attack-map').setView([30, 0], 2);
        L.tileLayer('https://{{s}}.basemaps.cartocdn.com/dark_all/{{z}}/{{x}}/{{y}}{{r}}.png', {{
            attribution: '&copy; CartoDB',
            maxZoom: 19
        }}).addTo(map);

        var processed = {{}};
        attempts.forEach(function(a) {{
            if (processed[a.ip]) return;
            processed[a.ip] = true;

            fetch('https://ipapi.co/' + a.ip + '/json/')
                .then(function(r) {{ return r.json(); }})
                .then(function(geo) {{
                    if (geo.latitude && geo.longitude) {{
                        var marker = L.circleMarker([geo.latitude, geo.longitude], {{
                            radius: 8,
                            fillColor: '#ff4444',
                            color: '#ff0000',
                            weight: 2,
                            opacity: 1,
                            fillOpacity: 0.7
                        }}).addTo(map);

                        marker.bindPopup(
                            '<strong>' + a.ip + '</strong><br>' +
                            'Source: ' + a.source + '<br>' +
                            'Time: ' + a.time + '<br>' +
                            'Location: ' + (geo.city || 'Unknown') + ', ' + (geo.country_name || 'Unknown')
                        );
                    }}
                }})
                .catch(function() {{}});
        }});
    }})();
    </script>
</body>
</html>"#,
        contacts.len(),
        service_inquiries.len(),
        honeypot_attempts.len(),
        contacts.len() + service_inquiries.len(),
        contact_rows,
        service_rows,
        honeypot_rows,
        honeypot_ips_json
    )
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        message: "Server is running".to_string(),
    })
}

async fn serve_embedded(req: HttpRequest) -> HttpResponse {
    let path = req.path().trim_start_matches('/');
    serve_file(path)
}

async fn serve_index() -> HttpResponse {
    serve_file("index.html")
}

fn get_cache_header(path: &str) -> &'static str {
    if path.ends_with(".webp") || path.ends_with(".png") || path.ends_with(".ico") || path.ends_with(".jpg") {
        "public, max-age=31536000, immutable"
    } else if path.ends_with(".min.css") || path.ends_with(".min.js") {
        "public, max-age=31536000, immutable"
    } else if path.ends_with(".css") || path.ends_with(".js") {
        "public, max-age=604800"
    } else if path.ends_with(".html") {
        "public, max-age=3600"
    } else {
        "public, max-age=86400"
    }
}

fn serve_file(path: &str) -> HttpResponse {
    let path = path.trim_start_matches('/');

    // Try exact path first
    if let Some(content) = Assets::get(path) {
        let mime = from_path(path).first_or_octet_stream();
        let cache_header = get_cache_header(path);
        return HttpResponse::Ok()
            .content_type(mime.as_ref())
            .insert_header((header::CACHE_CONTROL, cache_header))
            .body(content.data.into_owned());
    }

    // If no extension, try adding .html (clean URLs)
    if !path.contains('.') && !path.is_empty() {
        let html_path = format!("{}.html", path);
        if let Some(content) = Assets::get(&html_path) {
            return HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .insert_header((header::CACHE_CONTROL, "public, max-age=3600"))
                .body(content.data.into_owned());
        }
    }

    // Try index.html for directories
    if path.ends_with('/') || !path.contains('.') {
        let index_path = if path.is_empty() {
            "index.html".to_string()
        } else {
            format!("{}/index.html", path.trim_end_matches('/'))
        };

        if let Some(content) = Assets::get(&index_path) {
            return HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .insert_header((header::CACHE_CONTROL, "public, max-age=3600"))
                .body(content.data.into_owned());
        }
    }

    HttpResponse::NotFound()
        .content_type("text/html; charset=utf-8")
        .body(generate_not_found_html())
}

fn list_embedded_assets() {
    println!("\nEmbedded assets:");
    for file in Assets::iter() {
        if let Some(asset) = Assets::get(&file) {
            println!("  {} ({} bytes)", file, asset.data.len());
        }
    }
    println!();
}

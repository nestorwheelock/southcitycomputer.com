use actix_cors::Cors;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, middleware, http::header};
use base64::Engine;
use chrono::Local;
use mime_guess::from_path;
use printpdf::*;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;
use uuid::Uuid;

// Include all the shared code
include!("shared.rs");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "9000".to_string());
    let bind_addr = format!("0.0.0.0:{}", port);

    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║     SOUTH CITY COMPUTER - Single Binary Web Server        ║");
    println!("╠═══════════════════════════════════════════════════════════╣");
    println!("║  All assets embedded in binary - zero disk reads          ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!();
    println!("Starting server on http://{}", bind_addr);

    list_embedded_assets();

    println!("API Endpoints:");
    println!("  POST /api/contact         - Submit contact form");
    println!("  POST /api/service-inquiry - Submit service intake form");
    println!("  GET  /view/{{id}}           - View submission");
    println!("  GET  /view/{{id}}/pdf       - Download PDF");
    println!("  GET  /whitepaper/pdf/{{lang}} - Download whitepaper PDF");
    println!("  GET  /contact-admin       - Admin panel (auth required)");
    println!("  GET  /health              - Health check");
    println!("  GET  /*                   - Embedded static files");
    println!();

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .route("/api/contact", web::post().to(handle_contact))
            .route("/api/service-inquiry", web::post().to(handle_service_inquiry))
            .route("/view/{id}", web::get().to(view_submission))
            .route("/view/{id}/pdf", web::get().to(download_pdf))
            .route("/whitepaper/pdf/{lang}", web::get().to(download_whitepaper_pdf))
            .route("/contact-admin", web::get().to(contact_admin))
            .route("/health", web::get().to(health_check))
            .route("/", web::get().to(serve_index))
            .default_service(web::get().to(serve_embedded))
    })
    .bind(&bind_addr)?
    .run()
    .await
}

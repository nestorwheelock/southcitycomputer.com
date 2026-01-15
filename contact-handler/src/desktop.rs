// South City Computer - Desktop Application
// Uses wry/tao to wrap the web server in a native window

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
use std::net::TcpListener;
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use sys_locale::get_locale;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use tokio;
use uuid::Uuid;
use wry::WebViewBuilder;

// Include all the shared code from main.rs
include!("shared.rs");

fn find_available_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind to port")
        .local_addr()
        .expect("Failed to get local address")
        .port()
}

fn main() {
    let port = find_available_port();

    // Detect system language
    let system_lang = get_locale()
        .map(|l| if l.starts_with("es") { "es" } else { "en" })
        .unwrap_or("en");

    // Pass system language as URL parameter for initial load
    let url = format!("http://127.0.0.1:{}/?syslang={}", port, system_lang);

    // Channel to signal when server is ready
    let (tx, rx) = mpsc::channel();

    // Start web server in background thread
    let server_url = url.clone();
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let bind_addr = format!("127.0.0.1:{}", port);

            println!("╔═══════════════════════════════════════════════════════════╗");
            println!("║     SOUTH CITY COMPUTER - Desktop Application             ║");
            println!("╠═══════════════════════════════════════════════════════════╣");
            println!("║  All assets embedded in binary - zero disk reads          ║");
            println!("╚═══════════════════════════════════════════════════════════╝");
            println!();
            println!("Starting local server on http://{}", bind_addr);

            let server = HttpServer::new(|| {
                let cors = Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600);

                App::new()
                    .wrap(cors)
                    .wrap(middleware::Compress::default())
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
            .bind(&bind_addr)
            .expect("Failed to bind server");

            // Signal that server is ready
            tx.send(()).unwrap();

            server.run().await.unwrap();
        });
    });

    // Wait for server to be ready
    rx.recv().unwrap();

    // Give server a moment to fully initialize
    thread::sleep(std::time::Duration::from_millis(100));

    // Create native window with webview
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("South City Computer")
        .with_inner_size(tao::dpi::LogicalSize::new(1200.0, 800.0))
        .build(&event_loop)
        .expect("Failed to create window");

    let _webview = WebViewBuilder::new(&window)
        .with_url(&url)
        .with_devtools(cfg!(debug_assertions))
        .build()
        .expect("Failed to create webview");

    println!("Desktop app ready at {}", url);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("Window closed, exiting...");
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}

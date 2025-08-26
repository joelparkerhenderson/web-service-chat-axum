//! # Web service chat axum
//!
//! **[documentation](https://docs.rs/web-service-chat-axum/)** 
//! •
//! **[source](https://github.com/joelparkerhenderson/web-service-chat-axum/)**
//! •
//! **[llms.txt](https://raw.githubusercontent.com/joelparkerhenderson/web-service-chat-axum/refs/heads/main/llms.txt)**
//! • 
//! **[crate](https://crates.io/crates/web-service-chat-axum)** 
//! •
//! **[email](mailto:joel@joelparkerhenderson.com)**
//!
//! Web service that interacts with OpenAI GPT-OSS chat by using Axum, Tokio, Rust.
//! 
//! The purpose of this is simple testing of our systems.
//!
//! ## Steps
//!
//! Run the service using the default address 0.0.0.0:8080:
//!
//! ```sh
//! cargo run
//! ```
//!
//! Browse <https://localhost:8080/gpt?sunshine>
//!
//! You should see a web page with a chat about sunshine.
//!
//! ## Options
//!
//! Run the service using an environment variable for a custom bind address:
//! 
//! ```sh
//! export BIND="1.1.1.1:1111"
//! cargo run
//! ```
//! 
//! Run the service using environment variables for a custom host and port:
//! 
//! ```sh
//! export HOST="1.1.1.1"
//! export PORT="1111"
//! cargo run
//! ```
//!
//! ## References
//!
//! Based on Demo Rust Axum free open source software:
//! <https://github.com/joelparkerhenderson/demo-rust-axum>
//!

mod app;
mod conf;

/// Use tracing crates for application-level tracing output.
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// The main function does these steps:
/// - Start tracing and emit a tracing event.
/// - Get a command line argument as our bind address.
/// - Build our application by creating our router.
/// - Run our application as a hyper server.
#[tokio::main]
async fn main() {
    // Start tracing and emit a tracing event.
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
    tracing::event!(tracing::Level::INFO, "main");

    // Create our application which is an axum router.
    let app = crate::app::app();

    // Run our application as a hyper server.
    let listener = tokio::net::TcpListener::bind(crate::conf::bind_string().await).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(crate::conf::shutdown_signal())
        .await
        .unwrap();
}

use crate::settings::Settings;
use crate::state::ApplicationState;
use clap::{value_parser, Arg, ArgMatches, Command};
use sea_orm::Database;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};

use opentelemetry::logs::LogError;
use opentelemetry::trace::TraceError;
use opentelemetry::{global, KeyValue};
//use opentelemetry::KeyValue;
use opentelemetry_otlp::{ExportConfig, WithExportConfig};
use opentelemetry_sdk::logs::Config;
use opentelemetry_sdk::metrics::MeterProvider;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::{runtime, trace as sdktrace, Resource};
use std::str::FromStr;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;

use anyhow::Context;

use tracing_subscriber::Registry;

/// Starts a server with a default port of 8080
pub fn configure() -> Command {
    Command::new("serve").about("Start an HTTP server").arg(
        Arg::new("port")
            .short('p')
            .long("port")
            .value_name("PORT")
            .help("TCP port to listen on")
            //.default_value("8080")
            .default_value("3000")
            .value_parser(value_parser!(u16)),
    )
}

pub fn handle(matches: &ArgMatches, settings: &Settings) -> anyhow::Result<()> {
    if let Some(matches) = matches.subcommand_matches("serve") {
        //let port: u16 = *matches.get_one("port").unwrap_or(&8080);
        let port: u16 = *matches.get_one("port").expect("Default set by parser");

        start_tokio(port, settings)?;
    }

    Ok(())
}

fn start_tokio(port: u16, settings: &Settings) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("Failed to build Tokio runtime")?
        .block_on(async move {

            // Sets log level from .evn file or defaults to DEBUG
            let log_level = settings
                .logging
                .log_level
                .as_deref()
                .and_then(|lvl| LevelFilter::from_str(lvl).ok())
                .unwrap_or(LevelFilter::DEBUG);

            // If an OTLP endpoint is provided in the .env file:
            //   - Initializes span propagator for distributed tracing
            //   - Initializes OTLP tracing, logging, and metrics exporters
            //   - Initializes subscriber with OTLP layer & default console logging
            // else: 
            //   - Initializes subscriber with pretty-print logging to console only
            // NOTE: Default OTLP/gRPC endpoint for SigNoz: http://localhost:4317
            // TODO: Get SigNoz logging & metrics working
            if let Some(otlp_endpoint) = settings.tracing.otlp_endpoint.clone() {

                // Initializes global otel span propagator
                global::set_text_map_propagator(TraceContextPropagator::new());

                // Initializes otel providers
                let tracer = init_tracer(&otlp_endpoint)?;
                let _ = init_metrics(&otlp_endpoint);
                let _ = init_logs(&otlp_endpoint);

                // Create OTLP tracing layer
                let tracing_layer = tracing_opentelemetry::layer()
                    .with_tracer(tracer);

                // Configurees the subscriber with the tracing layer
                let subscriber = Registry::default()
                    .with(log_level)
                    .with(fmt::Layer::default()) // Basic console logging
                    .with(tracing_layer); // OTLP span exporter

                // Initializes the subscriber
                tracing::subscriber::set_global_default(subscriber)
                    .context("Failed to set global tracing subscriber")?;

            } else {

                // Configures the subscriber with pretty console output
                let subscriber = Registry::default()
                    .with(log_level)
                    .with(fmt::Layer::default()
                        .with_writer(std::io::stdout).pretty());

                // Initializes the subscriber
                tracing::subscriber::set_global_default(subscriber)
                    .context("Failed to set global tracing subscriber")?;
            }

            // Initialize DB connection and app state
            let db_url = settings
                .database
                .url
                .clone()
                .context("Missing database URL")?;

            let db_conn: sea_orm::DatabaseConnection = Database::connect(db_url)
                .await
                .expect("Database connection failed");

            let state = Arc::new(ApplicationState::new(settings, db_conn)?);

            // Configures Axum server with localhost, user-defined port,
            // and defines the API endpoints
            let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);
            let routes = crate::api::configure(state).layer(
                TraceLayer::new_for_http()
                    .on_request(DefaultOnRequest::new())
                    .on_response(DefaultOnResponse::new()),
            );

            // Off to the traces!
            tracing::info!("starting Axum on port {}", port);

            // Starts the Axum server
            axum::Server::bind(&addr)
                .serve(routes.into_make_service())
                .await?;

            Ok::<(), anyhow::Error>(())
        })?;

    std::process::exit(0);
}

fn init_tracer(otlp_endpoint: &str) -> Result<sdktrace::Tracer, TraceError> {
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(otlp_endpoint),
        )
        .with_trace_config(
            sdktrace::config().with_resource(Resource::new(vec![KeyValue::new(
                "service.name",
                "docserve",
            )])),
        )
        .install_batch(runtime::Tokio)
}

fn init_metrics(otlp_endpoint: &str) -> opentelemetry::metrics::Result<MeterProvider> {
    let export_config = ExportConfig {
        endpoint: otlp_endpoint.to_string(),
        ..ExportConfig::default()
    };
    opentelemetry_otlp::new_pipeline()
        .metrics(runtime::Tokio)
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_export_config(export_config),
        )
        .with_resource(Resource::new(vec![KeyValue::new(
            opentelemetry_semantic_conventions::resource::SERVICE_NAME,
            "docserve",
        )]))
        .build()
}

fn init_logs(otlp_endpoint: &str) -> Result<opentelemetry_sdk::logs::Logger, LogError> {
    opentelemetry_otlp::new_pipeline()
        .logging()
        .with_log_config(
            Config::default().with_resource(Resource::new(vec![KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                "docserve",
            )])),
        )
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(otlp_endpoint.to_string()),
        )
        .install_batch(runtime::Tokio)
}

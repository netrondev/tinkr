use opentelemetry::{KeyValue, global};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    Resource,
    metrics::{
        MeterProviderBuilder, PeriodicReader, SdkMeterProvider,
        reader::{DefaultAggregationSelector, DefaultTemporalitySelector},
    },
    runtime,
    trace::{BatchConfig, Config as TraceConfig, RandomIdGenerator, Sampler, Tracer},
};
use opentelemetry_semantic_conventions::{
    SCHEMA_URL,
    resource::{DEPLOYMENT_ENVIRONMENT, SERVICE_NAME, SERVICE_VERSION},
};
use pyroscope::pyroscope::PyroscopeAgentRunning;
use tracing_loki::BackgroundTask;

/// Configuration for telemetry setup
pub struct TelemetryConfig {
    pub service_name: String,
    pub service_version: String,
    pub service_authors: String,
    pub service_description: String,
    pub deployment_environment: String,
    pub otel_endpoint: String,
    pub loki_url: String,
    pub pyroscope_url: String,
    pub log_filter: String,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            service_name: env!("CARGO_PKG_NAME").to_string(),
            service_version: env!("CARGO_PKG_VERSION").to_string(),
            service_authors: env!("CARGO_PKG_AUTHORS").to_string(),
            service_description: env!("CARGO_PKG_DESCRIPTION").to_string(),
            deployment_environment: std::env::var("DEPLOYMENT_ENVIRONMENT")
                .unwrap_or_else(|_| "develop".to_string()),
            otel_endpoint: std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
                .unwrap_or_else(|_| "http://tempo:4317".to_string()),
            loki_url: std::env::var("TELEMETRY_LOKI_URL")
                .unwrap_or_else(|_| "http://loki:3100".to_string()),
            pyroscope_url: std::env::var("TELEMETRY_PYROSCOPE_URL")
                .unwrap_or_else(|_| "http://pyroscope:4040".to_string()),
            log_filter: std::env::var("RUST_LOG").unwrap_or_else(|_| {
                "info,app=debug,tower_http=debug,axum::rejection=trace".to_string()
            }),
        }
    }
}

// Create a Resource that captures information about the entity for which telemetry is recorded.
fn resource(config: &TelemetryConfig) -> Resource {
    Resource::from_schema_url(
        [
            KeyValue::new(SERVICE_NAME, config.service_name.clone()),
            KeyValue::new(SERVICE_VERSION, config.service_version.clone()),
            KeyValue::new(
                DEPLOYMENT_ENVIRONMENT,
                config.deployment_environment.clone(),
            ),
        ],
        SCHEMA_URL,
    )
}

/// Construct MeterProvider for MetricsLayer
pub fn init_meter_provider(config: &TelemetryConfig) -> SdkMeterProvider {
    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(config.otel_endpoint.clone())
        .with_timeout(std::time::Duration::from_secs(2))
        .build_metrics_exporter(
            Box::new(DefaultAggregationSelector::new()),
            Box::new(DefaultTemporalitySelector::new()),
        )
        .unwrap();

    let reader = PeriodicReader::builder(exporter, runtime::Tokio)
        .with_interval(std::time::Duration::from_secs(15))
        .build();

    // For debugging in development
    let stdout_reader = PeriodicReader::builder(
        opentelemetry_stdout::MetricsExporter::default(),
        runtime::Tokio,
    )
    .build();

    let meter_provider = MeterProviderBuilder::default()
        .with_resource(resource(config))
        .with_reader(reader)
        .with_reader(stdout_reader)
        .build();

    global::set_meter_provider(meter_provider.clone());

    meter_provider
}

/// Construct Tracer for OpenTelemetryLayer
pub fn init_tracer(config: &TelemetryConfig) -> Tracer {
    eprintln!(
        "Initializing OpenTelemetry tracer with endpoint: {}",
        config.otel_endpoint
    );

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(
            TraceConfig::default()
                .with_sampler(Sampler::AlwaysOn)
                .with_id_generator(RandomIdGenerator::default())
                .with_resource(resource(config)),
        )
        .with_batch_config(BatchConfig::default())
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(config.otel_endpoint.clone())
                .with_timeout(std::time::Duration::from_secs(2)),
        )
        .install_batch(runtime::Tokio)
        .expect(&format!(
            "Failed to initialize OpenTelemetry tracer with endpoint: {}",
            config.otel_endpoint
        ));

    eprintln!("OpenTelemetry tracer initialized successfully");
    tracer
}

/// Initialize Loki layer for logs
pub fn init_loki_layer(
    config: &TelemetryConfig,
) -> Result<(tracing_loki::Layer, BackgroundTask), Box<dyn std::error::Error + Send + Sync>> {
    eprintln!("Initializing Loki layer with endpoint: {}", config.loki_url);

    let (layer, task) = tracing_loki::builder()
        .label("app", &config.service_name)?
        .label("environment", &config.deployment_environment)?
        .extra_field("version", &config.service_version)?
        .build_url(tracing_loki::url::Url::parse(&config.loki_url)?)?;

    eprintln!("Loki layer initialized successfully");
    Ok((layer, task))
}

/// Initialize Pyroscope profiling agent
pub fn init_pyroscope(
    config: &TelemetryConfig,
) -> Result<pyroscope::PyroscopeAgent<PyroscopeAgentRunning>, String> {
    eprintln!(
        "Initializing Pyroscope agent with endpoint: {}",
        config.pyroscope_url
    );

    let agent = pyroscope::PyroscopeAgent::builder(
        config.pyroscope_url.clone(),
        config.service_name.clone(),
    )
    .backend(pyroscope_pprofrs::pprof_backend(
        pyroscope_pprofrs::PprofConfig::new().sample_rate(100),
    ))
    .build()
    .map_err(|e| e.to_string())?
    .start()
    .map_err(|e| e.to_string())?;

    eprintln!("Pyroscope agent started successfully");
    Ok(agent)
}

/// Initialize complete telemetry stack (tracing, metrics, logging, profiling, panic hooks)
///
/// This is a convenience function that sets up:
/// - Pyroscope profiling
/// - OpenTelemetry tracing and metrics
/// - Loki logging
/// - Panic hook for error logging
/// - Startup tracing span
///
/// Returns the Pyroscope agent (keep it alive for the lifetime of your app)
pub async fn init_full_telemetry(
    config: TelemetryConfig,
) -> Result<
    pyroscope::PyroscopeAgent<PyroscopeAgentRunning>,
    Box<dyn std::error::Error + Send + Sync>,
> {
    use tracing_opentelemetry::{MetricsLayer, OpenTelemetryLayer};
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    // Initialize Pyroscope profiling
    let agent = init_pyroscope(&config)?;

    // Initialize OpenTelemetry
    let meter_provider = init_meter_provider(&config);
    let tracer = init_tracer(&config);

    // Initialize Loki layer for logs
    let (loki_layer, loki_task) = init_loki_layer(&config)?;
    tokio::spawn(loki_task);

    // Set up tracing subscriber with all layers
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| config.log_filter.clone().into()),
        )
        .with(MetricsLayer::new(meter_provider))
        .with(OpenTelemetryLayer::new(tracer))
        .with(loki_layer)
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Set up panic hook to log panics
    std::panic::set_hook(Box::new(|panic_info| {
        let location = panic_info
            .location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_else(|| "unknown".to_string());
        let payload = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            s
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.as_str()
        } else {
            "Unknown panic payload"
        };

        tracing::error!(
            location = %location,
            payload = %payload,
            "Panic occurred"
        );
    }));

    // Log startup info
    tracing::info!(
        "{} v{} - {} by {}",
        config.service_name,
        config.service_version,
        config.service_description,
        config.service_authors
    );

    // Test OTLP connection with a trace span
    tracing::info!("Testing OpenTelemetry trace export...");
    let test_span = tracing::info_span!(
        "application_startup",
        app_name = config.service_name,
        version = config.service_version
    );
    let _enter = test_span.enter();
    tracing::info!("Application initialization complete - trace should be exported");
    drop(_enter);

    Ok(agent)
}

pub fn init_logging(
    log_level: &str,
) -> anyhow::Result<tracing_appender::non_blocking::WorkerGuard> {
    use tracing_appender::rolling;
    use tracing_subscriber::{prelude::*, EnvFilter};

    let file_appender = rolling::daily("logs", "app.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let file_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(non_blocking)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL);

    let console_layer = tracing_subscriber::fmt::layer().pretty().with_target(false);

    let filter = EnvFilter::new(log_level);

    tracing_subscriber::registry()
        .with(filter)
        .with(console_layer)
        .with(file_layer)
        .try_init()?;

    Ok(guard)
}

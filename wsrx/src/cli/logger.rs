use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logger() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "wsrx=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

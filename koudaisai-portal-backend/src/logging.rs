use crate::config::Logging;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init_logging(logging: Logging) {
    if logging.json {
        tracing_subscriber::registry()
            .with(logging.log_level.to_level_filter())
            .with(tracing_subscriber::fmt::layer().json())
            .init();
    } else {
        tracing_subscriber::registry()
            .with(logging.log_level.to_level_filter())
            .with(tracing_subscriber::fmt::layer())
            .init();
    }
}

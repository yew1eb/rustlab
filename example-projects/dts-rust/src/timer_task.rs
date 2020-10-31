use tokio::time::interval;
use std::time::{Duration, Instant};

/// Execute  every 3 second.
pub async fn execute() {
    let since = Instant::now();
    let mut interval = interval(Duration::from_secs(3));
    loop {
        interval.tick().await;
        println!(
            "uptime_seconds {}, execute task",
            since.elapsed().as_secs() as f64
        );
    }
}

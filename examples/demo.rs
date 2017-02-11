#[macro_use]
extern crate slog;
extern crate slog_extra;
extern crate slog_term;
extern crate slog_slack;

use slog::*;

fn main() {
    let slack_webhook_url = std::env::args().nth(1)
        .expect("Usage: cargo run --example WEBHOOK_URL");

    // Send log output to both stdout and Slack
    let drain = slog::duplicate(
        slog_term::streamer().async().full().build(),

        // Use LevelFilter to make sure only the most important messages go to Slack
        LevelFilter::new(
            // Wrap SlackWebhook in Async to avoid blocking for the HTTP request to
            // complete for every log message
            slog_extra::Async::new(
                slog_slack::SlackWebhook::new(&slack_webhook_url).fuse()
            ),
            slog::Level::Info
        ),
    );

    let log = Logger::root(drain.fuse(), o!());
    crit! (log, "Example log message at critical level");
    error!(log, "Example log message at error level");
    warn! (log, "Example log message at warning level");
    info! (log, "Example log message at info level");
    debug!(log, "Example log message at debug level");
    trace!(log, "Example log message at trace level");

    // Process will not terminate until all messages have been drained to Slack
}

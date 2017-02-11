extern crate slog;
extern crate slack_hook;

use slog::*;
use slack_hook::{Slack, PayloadBuilder, AttachmentBuilder};

pub struct SlackWebhook {
    slack: Slack,
}

impl SlackWebhook {
    pub fn new(webhook_url: &str) -> SlackWebhook {
        SlackWebhook {
            slack: Slack::new(webhook_url).unwrap()
        }
    }
}

fn color_for_loglevel(level: Level) -> Option<&'static str> {
    use Level::*;
    match level {
        Critical => Some("#000000"),
        Error => Some("danger"),
        Warning => Some("warning"),
        Info => None,
        Debug => Some("good"),
        Trace => Some("#ffff00"),
    }
}

fn colorize_for_loglevel(builder: AttachmentBuilder, level: Level) -> AttachmentBuilder {
    match color_for_loglevel(level) {
        Some(color) => builder.color(color),
        None => builder,
    }
}

impl Drain for SlackWebhook {
    type Error = slack_hook::Error;

    fn log(&self, info: &Record, _attributes: &OwnedKeyValueList) -> Result<(), Self::Error> {
        // _attributes is completely inaccessible, because slog has decided to use
        // its very own serializer which I can't be assed to implement support for

        let p = PayloadBuilder::new()
            .attachments(vec![
                colorize_for_loglevel(
                    AttachmentBuilder::new(std::fmt::format(info.msg())),
                    info.level()
                )
                    .build()
                    .unwrap()
            ])
            .build()
            .unwrap();

        self.slack.send(&p)
    }
}

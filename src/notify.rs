use notify_rust::{Notification, Timeout};

use crate::sound::play_notification_sound;

pub struct Reminder<'a> {
    pub message: &'a str,
    pub timeout: u32,
    pub app_name: &'a str,
    pub sound: bool,
}

impl<'a> Reminder<'a> {
    pub fn send(&self) -> Result<(), Box<dyn std::error::Error>> {
        let timeout = if self.timeout == 0 {
            Timeout::Never
        } else {
            Timeout::Milliseconds(self.timeout)
        };

        Notification::new()
            .appname(self.app_name)
            .summary("Hydration Reminder")
            .body(self.message)
            .timeout(timeout)
            .icon("water")
            .show()?;

        if self.sound {
            play_notification_sound()?;
        }
        println!("[{}] Notification sent", chrono_now());

        Ok(())
    }
}

fn chrono_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();
    let hrs = (secs / 3600) % 24;
    let mins = (secs / 60) % 60;
    format!("{:02}:{:02}", hrs, mins)
}

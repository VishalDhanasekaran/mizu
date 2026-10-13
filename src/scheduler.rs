use std::thread;
use std::time::Duration;

use crate::notify::Reminder;

pub fn run(interval_minutes: u64, reminder: &Reminder) -> Result<(), Box<dyn std::error::Error>> {
    let interval_secs = interval_minutes * 60;

    println!("💧 Hydration reminder started");
    println!("   Interval: {} minutes", interval_minutes);
    println!("   Press Ctrl+C to stop\n");

    loop {
        if let Err(e) = reminder.send() {
            eprintln!("warning: could not send notification: {e}");
        }
        thread::sleep(Duration::from_secs(interval_secs));
    }
}

use clap::Parser;
use notify_rust::{Notification, Timeout};
use std::thread;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(name = "hydrate", about = "Stay hydrated with regular reminders")]
#[command(version)]
struct Args {
    #[arg(short, long, default_value_t = 30, value_name = "MINUTES")]
    interval: u64,

    #[arg(short, long, default_value = "Time to drink water! 💧")]
    message: String,

    /// Notification timeout in milliseconds (0 = persistent)
    #[arg(short, long, default_value_t = 5000)]
    timeout: u32,

    #[arg(short, long, default_value = "Hydrate")]
    app_name: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let interval_secs = args.interval * 60;

    println!("💧 Hydration reminder started");
    println!("   Interval: {} minutes", args.interval);
    println!("   Press Ctrl+C to stop\n");

    send_notification(&args)?;

    loop {
        thread::sleep(Duration::from_secs(interval_secs));
        send_notification(&args)?;
    }
}

fn send_notification(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let timeout = if args.timeout == 0 {
        Timeout::Never
    } else {
        Timeout::Milliseconds(args.timeout)
    };

    Notification::new()
        .appname(&args.app_name)
        .summary("Hydration Reminder")
        .body(&args.message)
        .timeout(timeout)
        .icon("water")
        .show()?;

    println!("[{}] Notification sent", chrono_now());

    Ok(())
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

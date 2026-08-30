mod notify;
mod scheduler;
mod sound;

use clap::Parser;

use notify::Reminder;
use scheduler::run;

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
    let reminder = Reminder {
        message: &args.message,
        timeout: args.timeout,
        app_name: &args.app_name,
    };

    run(args.interval, &reminder)
}

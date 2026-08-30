mod config;
mod notify;
mod scheduler;
mod sound;

use clap::Parser;
use std::os::fd::AsRawFd;
use std::process;

use notify::Reminder;
use scheduler::run;

const DAEMON_FLAG: &str = "--background";

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
    if std::env::args().any(|a| a == DAEMON_FLAG) {
        return run_daemon();
    }
    setup_runner()
}

fn run_daemon() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let cfg = config::load();
    let reminder = Reminder {
        message: &args.message,
        timeout: args.timeout,
        app_name: &args.app_name,
        sound: cfg.sound,
    };
    run(args.interval, &reminder)
}

fn setup_runner() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let sound = prompt_sound(config::load().sound);

    let pid = daemonize();
    if pid > 0 {
        println!("💧 Hydration reminder running in background");
        println!("   PID: {}", pid);
        println!("   Interval: {} minutes", args.interval);
        println!("   Sound: {}", if sound { "enabled" } else { "disabled" });
        return Ok(());
    }

    config::save(sound);

    let reminder = Reminder {
        message: &args.message,
        timeout: args.timeout,
        app_name: &args.app_name,
        sound,
    };
    run(args.interval, &reminder)
}

#[cfg(not(feature = "sound"))]
fn prompt_sound(current: bool) -> bool {
    if current {
        eprintln!("note: sound support not compiled into this build");
    }
    false
}

#[cfg(feature = "sound")]
fn prompt_sound(current: bool) -> bool {
    use std::io::Write;

    let default = if current { "Y/n" } else { "y/N" };
    print!("Need notification sound? [{default}] ");
    std::io::stdout().flush().ok();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    let input = input.trim().to_ascii_lowercase();

    match input.as_str() {
        "" => current,
        "y" | "yes" => true,
        _ => false,
    }
}

fn daemonize() -> u32 {
    let pid = unsafe { libc::fork() };
    if pid > 0 {
        return pid as u32;
    }

    if pid < 0 {
        eprintln!("error: could not fork");
        process::exit(1);
    }

    unsafe {
        libc::setsid();
    }
    redirect_to_devnull(libc::STDIN_FILENO);
    redirect_to_devnull(libc::STDOUT_FILENO);
    redirect_to_devnull(libc::STDERR_FILENO);

    0
}

fn redirect_to_devnull(fd: i32) {
    let null = std::fs::File::open("/dev/null").expect("open /dev/null");
    unsafe {
        libc::dup2(null.as_raw_fd(), fd);
    }
}

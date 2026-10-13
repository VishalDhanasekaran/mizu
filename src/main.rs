mod config;
mod notify;
mod scheduler;
mod sound;

use clap::Parser;
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};

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

    /// Internal: run as the detached reminder process
    #[arg(long, hide = true)]
    background: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if args.background {
        return run_daemon(args);
    }
    setup_runner(args)
}

fn run_daemon(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let cfg = config::load();
    let reminder = Reminder {
        message: &args.message,
        timeout: args.timeout,
        app_name: &args.app_name,
        sound: cfg.sound,
    };
    run(args.interval, &reminder)
}

fn setup_runner(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let sound = prompt_sound(config::load().sound);
    config::save(sound);

    let pid = daemonize()?;
    println!("💧 Hydration reminder running in background");
    println!("   PID: {}", pid);
    println!("   Interval: {} minutes", args.interval);
    println!("   Sound: {}", if sound { "enabled" } else { "disabled" });
    Ok(())
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

/// Re-launch this binary detached from the terminal. A fresh exec (rather
/// than a bare fork) is required on macOS, where system frameworks used for
/// notifications are not fork-safe.
fn daemonize() -> std::io::Result<u32> {
    let mut cmd = Command::new(std::env::current_exe()?);
    cmd.args(std::env::args_os().skip(1))
        .arg(DAEMON_FLAG)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    unsafe {
        cmd.pre_exec(|| {
            libc::setsid();
            Ok(())
        });
    }
    Ok(cmd.spawn()?.id())
}

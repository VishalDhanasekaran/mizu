#[cfg(all(feature = "sound", target_os = "macos"))]
const SOUND_FILE: &str = "/System/Library/Sounds/Glass.aiff";
#[cfg(all(feature = "sound", target_os = "macos"))]
const PLAYERS: &[&str] = &["afplay"];

#[cfg(all(feature = "sound", not(target_os = "macos")))]
const SOUND_FILE: &str = "/usr/share/sounds/freedesktop/stereo/bell.oga";
#[cfg(all(feature = "sound", not(target_os = "macos")))]
const PLAYERS: &[&str] = &["pw-play", "paplay", "canberra-gtk-play", "aplay"];

#[cfg(feature = "sound")]
pub fn play_notification_sound() -> Result<(), Box<dyn std::error::Error>> {
    let player = PLAYERS
        .iter()
        .find(|p| command_exists(p))
        .ok_or_else(|| format!("no audio player found (tried {})", PLAYERS.join(", ")))?;

    let status = std::process::Command::new(player)
        .arg(SOUND_FILE)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()?;

    if !status.success() {
        return Err(format!("{player} exited with {status}").into());
    }
    Ok(())
}

#[cfg(not(feature = "sound"))]
pub fn play_notification_sound() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[cfg(feature = "sound")]
fn command_exists(cmd: &str) -> bool {
    std::process::Command::new("sh")
        .arg("-c")
        .arg(format!("command -v {cmd}"))
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

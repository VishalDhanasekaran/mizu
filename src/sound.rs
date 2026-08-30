const SOUND_FILE: &str = "/usr/share/sounds/freedesktop/stereo/bell.oga";
const PLAYERS: &[&str] = &["pw-play", "paplay", "canberra-gtk-play", "aplay"];

#[cfg(feature = "sound")]
pub fn play_notification_sound() -> Result<(), Box<dyn std::error::Error>> {
    let player = PLAYERS
        .iter()
        .find(|p| command_exists(p))
        .ok_or_else(|| "no audio player found (tried pw-play, paplay, canberra-gtk-play, aplay)")?;

    let status = std::process::Command::new(player)
        .arg(SOUND_FILE)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .ok()
        .and_then(|mut c| c.wait().ok());

    match status {
        Some(s) if s.success() => Ok(()),
        _ => Ok(()),
    }
}

#[cfg(not(feature = "sound"))]
pub fn play_notification_sound() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

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

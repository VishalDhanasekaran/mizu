use std::path::PathBuf;

pub struct Config {
    pub sound: bool,
}

pub fn config_path() -> PathBuf {
    let dir = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            std::env::var("HOME")
                .map(|h| PathBuf::from(h).join(".config"))
                .unwrap_or_else(|_| PathBuf::from("."))
        });
    dir.join("mizu").join("config")
}

pub fn load() -> Config {
    let sound = std::fs::read_to_string(config_path())
        .map(|s| s.trim() == "sound=yes")
        .unwrap_or(false);
    Config { sound }
}

pub fn save(sound: bool) {
    if let Some(dir) = config_path().parent() {
        if let Err(e) = std::fs::create_dir_all(dir) {
            eprintln!("warning: could not create config dir: {e}");
        }
    }
    let val = if sound { "yes" } else { "no" };
    if let Err(e) = std::fs::write(config_path(), format!("sound={val}\n")) {
        eprintln!("warning: could not save config: {e}");
    }
}

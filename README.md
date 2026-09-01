# mizu 💧

A simple hydration reminder CLI tool that keeps you drinking water throughout the day.

mizu runs quietly in the background and fires desktop notifications on a schedule you choose. Optionally, it can play a sound with each reminder.

## Features

- **Background daemon** — starts detached in the background and prints its PID, so your terminal is free immediately.
- **Desktop notifications** — powered by `notify-rust` (DBus), integrates with your system's notification daemon.
- **Optional sound** — plays a notification sound via your system audio player (`pw-play`, `paplay`, `canberra-gtk-play`, or `aplay`).
- **Persistent config** — remembers your sound preference between runs (`~/.config/mizu/config`).

## Requirements

- **Linux** or **macOS**
- Either **Nix** (with flakes) or the **Rust toolchain** to install
- A **desktop notification daemon** (e.g. `dunst`, `mako`, `xfce4-notifyd`, `swaync`)
- For sound: one of `pw-play`, `paplay`, `canberra-gtk-play`, or `aplay`, plus a sound file at `/usr/share/sounds/freedesktop/stereo/bell.oga`

## Installation

### Via Nix flake

```sh
# Run directly without installing
nix run github:VishalDhanasekaran/mizu -- -i 30

# Add to a NixOS/home-manager config
#   programs.package-name.enable = true;
```

> **Note:** `nix run github:owner/repo` may serve a cached old revision of the flake. Use `--refresh` to force fetching the latest:

```sh
nix run --refresh github:VishalDhanasekaran/mizu -- -i 30
```

To try a specific branch:

```sh
nix run github:VishalDhanasekaran/mizu/BRANCH_NAME -- -i 30
```

### Via cargo

```sh
cargo install --path .
```

or build a release binary:

```sh
cargo build --release
# binary at target/release/mizu
```

> The default build includes the `sound` feature. To build without sound: `cargo build --release --no-default-features`

### Via the install script

```sh
./install.sh
```

This builds a release binary and installs it to `~/.local/bin`, adding it to your `PATH` if needed.

## Usage

```sh
mizu [OPTIONS]
```

| Option | Description | Default |
| ------ | ----------- | ------- |
| `-i, --interval <MINUTES>` | Minutes between reminders | `30` |
| `-m, --message <MESSAGE>` | Notification body text | `"Time to drink water! 💧"` |
| `-t, --timeout <TIMEOUT>` | Notification timeout in ms (`0` = persistent) | `5000` |
| `-a, --app-name <APP_NAME>` | Notification app name | `Hydrate` |

### Examples

```sh
# Remind every 45 minutes
mizu -i 45

# Custom message that stays on screen
mizu -i 30 -m "Take a sip!" -t 0

# Custom notification name
mizu -a "DrinkUp" -i 60
```

On first launch mizu asks whether it should play a sound:

```text
Need notification sound? [y/N]
```

Answering is optional — pressing **Enter** keeps the previous preference (defaults to off). The choice is saved to `~/.config/mizu/config` (format: `sound=yes` or `sound=no`) and reused for all future reminders.

The command returns immediately. To stop the daemon, kill the printed PID:

```sh
kill <PID>
```

## How it works

1. mizu parses your options and (optionally) prompts for sound.
2. It **daemonizes** — detaches from the terminal, redirects stdio to `/dev/null`, and reports its PID.
3. In the background, the scheduler sleeps for the interval, then sends a notification (and plays the sound, if enabled).
4. Settings are persisted to `~/.config/mizu/config` so they survive restarts.

## Troubleshooting

- **No notifications appear** — make sure a desktop notification daemon is running (e.g. `dunst`, `mako`).
- **No sound** — ensure an audio player from the list is installed and `/usr/share/sounds/freedesktop/stereo/bell.oga` exists. mizu skips silently if neither is available.
- **Old behavior after updating** — the flake may be cached; run with `--refresh`.

## License

MIT

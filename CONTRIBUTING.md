# Contributing to mizu

Thanks for taking the time to contribute! This guide covers how to set up your environment, make changes, and get them merged.

## Development setup

### With Nix

The repository provides a dev shell with the Rust toolchain and all system dependencies:

```sh
nix develop
```

### Without Nix (cargo)

Make sure you have the [Rust toolchain](https://rustup.rs) installed, along with these system libraries:

- `pkg-config`
- `dbus`
- `libnotify`
- `openssl`

On Debian/Ubuntu:

```sh
sudo apt install pkg-config libdbus-1-dev libnotify-dev libssl-dev
```

## Building and running

```sh
# Type-check
cargo check

# Build (debug)
cargo build

# Build (release)
cargo build --release

# Run with a short interval for testing
cargo run -- -i 1
```

> **Note:** mizu daemonizes by default. In `cargo run`, the reported PID is the background daemon; kill it when you're done testing: `kill <PID>`.

### Feature flags

- `sound` (enabled by default) — plays a notification sound via a system audio player. It is designed to be **ALSA-free**, shelling out to `pw-play`/`paplay`/`canberra-gtk-play`/`aplay` instead of linking an audio library.
- Build without sound support: `cargo build --no-default-features`

## Code structure

| File | Purpose |
| ---- | ------- |
| `src/main.rs` | CLI entry point, argument parsing, sound prompt, daemonization |
| `src/scheduler.rs` | The reminder loop |
| `src/notify.rs` | Desktop notification logic |
| `src/sound.rs` | Playing the notification sound |
| `src/config.rs` | Loading/saving the persistent config |
| `flake.nix` | Nix flake for building and packaging |

## Conventions

- **Commits**: use [Conventional Commits](https://www.conventionalcommits.org/) (e.g. `feat:`, `fix:`, `docs:`, `chore:`, `build:`, `refactor:`).
- **Branching**: create a feature branch off `master` (e.g. `feat/my-change`), then open a pull request against `master`.
- **Style**: `cargo fmt` for formatting; keep changes focused and minimal.

## Testing your changes

- Run `cargo check` and `cargo build` to confirm compilation.
- For changes to behavior, run manually with a short interval to verify the notification fires and the daemon behaves correctly.
- If you touch the Nix packaging, verify with `nix build` and `nix flake check`.

## Opening a pull request

1. Fork the repository and create your branch.
2. Make your changes with clear, conventional commits.
3. Push the branch and open a pull request against `master`.
4. Describe what changed and any screenshots/commands you used to verify.

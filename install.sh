#!/bin/sh
#
# install.sh - build and install mizu to ~/.local/bin
#
# Usage:
#   ./install.sh              install to ~/.local/bin
#   PREFIX=/usr/local ./install.sh   install to a custom prefix
#

set -e

INSTALL_DIR="${PREFIX:-$HOME/.local}/bin"
REPO_DIR="$(cd "$(dirname "$0")" && pwd)"

echo "==> mizu installer"

# Locate the project dir (works whether run from repo root or via PATH)
if [ ! -f "$REPO_DIR/Cargo.toml" ]; then
    echo "error: could not find Cargo.toml (run this script from the repository root)" >&2
    exit 1
fi

cd "$REPO_DIR"

# ---- build ----
if command -v cargo >/dev/null 2>&1; then
    echo "==> Building release binary with cargo..."
    cargo build --release
    BIN="$REPO_DIR/target/release/mizu"
elif command -v nix >/dev/null 2>&1; then
    echo "==> Building with nix..."
    nix build
    BIN="$(readlink "$REPO_DIR/result")/bin/mizu"
else
    echo "error: neither 'cargo' nor 'nix' found; install the Rust toolchain (https://rustup.rs)" >&2
    exit 1
fi

if [ ! -x "$BIN" ]; then
    echo "error: built binary not found at $BIN" >&2
    exit 1
fi

# ---- install ----
mkdir -p "$INSTALL_DIR"
install -m 0755 "$BIN" "$INSTALL_DIR/mizu"
echo "==> Installed mizu to $INSTALL_DIR/mizu"

# ---- PATH (only auto-modify for user-local installs) ----
case "$INSTALL_DIR" in
    "$HOME"/*)
        case ":$PATH:" in
            *":$INSTALL_DIR:"*) ;;
            *)
                echo "==> $INSTALL_DIR is not on your PATH."
                RC=""
                for f in "$HOME/.zshrc" "$HOME/.bashrc" "$HOME/.profile"; do
                    [ -f "$f" ] && RC="$f" && break
                done
                if [ -n "$RC" ]; then
                    if ! grep -qs "$INSTALL_DIR" "$RC"; then
                        printf '\nexport PATH="%s:$PATH"\n' "$INSTALL_DIR" >> "$RC"
                        echo "==> Added '$INSTALL_DIR' to PATH in $RC"
                        echo "    Restart your shell or run: source $RC"
                    fi
                else
                    echo "    Add '$INSTALL_DIR' to your PATH manually."
                fi
                ;;
        esac
        ;;
    *)
        case ":$PATH:" in
            *":$INSTALL_DIR:"*) ;;
            *) echo "    Add '$INSTALL_DIR' to your PATH manually." ;;
        esac
        ;;
esac

echo "==> Done. Start mizu with: mizu -i 30"

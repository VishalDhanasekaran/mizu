{
  description = "Hydration reminder CLI tool";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default;
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "mizu";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          cargoBuildFlags = [ "--features" "sound" ];

          nativeBuildInputs = with pkgs; [
            pkg-config
            makeWrapper
          ];

          buildInputs = with pkgs; [
            dbus
            libnotify
            libpulseaudio
          ] ++ pkgs.lib.optionals stdenv.hostPlatform.isLinux [
            openssl
            # alsa-lib  # needed only when building with the `sound` feature (rodio)
          ];

          postInstall = ''
            wrapProgram $out/bin/mizu \
              --set LD_LIBRARY_PATH "${pkgs.libpulseaudio.out}/lib"
          '';

          meta = with pkgs.lib; {
            description = "A simple hydration reminder CLI tool";
            homepage = "https://github.com/VishalDhanasekaran/mizu";
            license = licenses.mit;
            platforms = platforms.linux ++ platforms.darwin;
            mainProgram = "mizu";
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            pkg-config
            dbus
            libnotify
            libpulseaudio
            alsa-lib
            cargo-watch
          ];

          shellHook = ''
            export LD_LIBRARY_PATH="${pkgs.libpulseaudio.out}/lib:$LD_LIBRARY_PATH"
            export DBUS_SESSION_BUS_ADDRESS="unix:path=$XDG_RUNTIME_DIR/bus"
          '';
        };
      }
    );
}

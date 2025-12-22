{}: let
  # Rolling updates, not deterministic.
  pkgs = import (fetchTarball "channel:nixpkgs-unstable") {};
in
  pkgs.mkShell {
    nativeBuildInputs = with pkgs; [cargo rustc gcc rustfmt clippy];
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  }

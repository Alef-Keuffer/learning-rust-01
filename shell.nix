{}: let
  # Rolling updates, not deterministic.
  pkgs = import (fetchTarball "https://channels.nixos.org/nixpkgs-unstable/nixexprs.tar.xz") {};
in
  pkgs.mkShell {
    nativeBuildInputs = with pkgs; [cargo rustc gcc rustfmt clippy];
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  }

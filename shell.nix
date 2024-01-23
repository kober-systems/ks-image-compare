{ profile ? "default" }:
let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs { overlays = [ (import sources.rust-overlay) ]; };
in
pkgs.mkShell {
  LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
    libGL
    libxkbcommon
    wayland
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
  ];
  nativeBuildInputs = with pkgs; [
    rust-bin.stable.latest.${profile}

    cmake
    pkg-config
    fontconfig
    libxkbcommon
    openssl
    xorg.libxcb

    # keep this line if you use bash
    bashInteractive
  ];
}


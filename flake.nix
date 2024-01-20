{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";

    naersk.url = "github:nix-community/naersk/master";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, utils, naersk, fenix, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        toolchain = with fenix.packages.${system};
          combine [
            stable.rustc
            stable.cargo
            stable.rustfmt
            stable.rust-analyzer
            targets.wasm32-unknown-unknown.stable.rust-std
          ];
        naerskLib = pkgs.callPackage naersk {
          rustc = toolchain;
          cargo = toolchain;
        };
        libPath = with pkgs;
          lib.makeLibraryPath [
            libGL
            libxkbcommon
            wayland
            xorg.libX11
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
          ];
      in
      {
        defaultPackage = naerskLib.buildPackage {
          pname = "double-pendulum";
          src = ./.;

          LD_LIBRARY_PATH = libPath;
        };

        devShell = with pkgs; mkShell {
          buildInputs = [ toolchain trunk ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
          LD_LIBRARY_PATH = libPath;
        };
      });
}

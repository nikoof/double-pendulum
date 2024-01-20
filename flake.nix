{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, utils, crane, fenix, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

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

        toolchain = with fenix.packages.${system};
          combine [
            stable.rustc
            stable.cargo
            stable.rustfmt
            stable.rust-analyzer
            targets.wasm32-unknown-unknown.stable.rust-std
          ];

        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

          src = pkgs.lib.cleanSourceWith {
            src = ./.;
            filter = path: type:
              (pkgs.lib.hasSuffix "\.html" path) ||
              (pkgs.lib.hasSuffix "\.scss" path) ||
              (pkgs.lib.hasInfix "/assets/" path) ||
              (craneLib.filterCargoSources path type);
          };

          commonArgs = {
            inherit src;
            strictDeps = true;
          };

          cargoArtifacts = craneLib.buildDepsOnly {
            inherit src;
            doCheck = false;
          };
      in rec {
        packages.default = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;

          LD_LIBRARY_PATH = libPath;
        });

        packages.gh-pages = craneLib.buildTrunkPackage (commonArgs // {
          inherit (pkgs) wasm-bindgen-cli;
          inherit cargoArtifacts;

          CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
        });

        apps.default = utils.lib.mkApp {
          drv = packages.default;
        };

        devShells.default = craneLib.devShell {
          packages = with pkgs; [
            trunk
          ];

          LD_LIBRARY_PATH = libPath;
        };
      });
}

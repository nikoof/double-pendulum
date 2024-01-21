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

        toolchainWindows = with fenix.packages.${system};
          combine [
            stable.rustc
            stable.cargo
            targets.x86_64-pc-windows-gnu.stable.rust-std
          ];

        craneLibWindows = (crane.mkLib pkgs).overrideToolchain toolchainWindows;

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

          trunkExtraBuildArgs = "--public-url double-pendulum/";

          CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
        });

        packages.windows = craneLibWindows.buildPackage (commonArgs // {
          inherit cargoArtifacts;

          depsBuildBuild = with pkgs; [
            pkgsCross.mingwW64.stdenv.cc
            pkgsCross.mingwW64.windows.pthreads
          ];

          doCheck = false;

          CARGO_BUILD_TARGET = "x86_64-pc-windows-gnu";
        });

        apps.default = utils.lib.mkApp {
          drv = packages.default;
        };

        devShells.default = craneLib.devShell {
          packages = with pkgs; [
            trunk
            nodePackages.conventional-changelog-cli
          ];

          LD_LIBRARY_PATH = libPath;
        };
      });
}

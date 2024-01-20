# Double Pendulum

This is a double pendulum demo I made for a physics project. Powered by [egui](https://github.com/emilk/egui).

# Building

## With Nix

The flake provides three packages.

```
packages
├───aarch64-darwin
├───aarch64-linux
├───x86_64-darwin
└───x86_64-linux
    ├───default: package 'double-pendulum-0.2.0'
    ├───gh-pages: package 'double-pendulum-trunk-0.2.0'
    └───windows: package 'double-pendulum-0.2.0'
```

To build for the host platform, simply use

```shell
nix build github:nikoof/double-pendulum
```

To build for windows, you need to cross-compile on a \*nix host. Alternatively, you can download prebuilt binaries from the [releases](https://github.com/nikoof/double-pendulum/releases).

```shell
nix build github:nikoof/double-pendulum#windows
```

To build and package for wasm32 using trunk

```shell
nix build github:nikoof/double-pendulum#gh-pages
```

## Manually

Untested. You need Rust and Cargo, and additionally, you need the following libraries installed (for egui):

- libGL
- libxkbcommon
- libwayland
- libX11
- libXcursor
- libXrandr
- libXi

# Credits

Inspired by [myPhysicsLab](https://www.myphysicslab.com/pendulum/double-pendulum-en.html).

# License

Copyright © 2024 Nicolas Bratoveanu. Licensed under the [MIT License](./LICENSE).

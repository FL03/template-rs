{
  description = "A base flake for Rust-based projects";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rustPlatform = pkgs.rustPlatform;
      in
      {
        packages.default = rustPlatform.buildRustPackage {
          pname = "{{ project-name }}";
          version = "{{ version }}";
          src = "self";
          # If Cargo.lock doesn't exist yet, remove or comment out this block:
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          doCheck = true;
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [
            pkgs.cargo-watch
            pkgs.cargo-nextest
            pkgs.clippy
            pkgs.rustfmt
          ];
          shellHook = ''
            echo "Welcome to the proton dev shell!"
            # Add any additional environment setup here
          '';
        };
      }
    );
}
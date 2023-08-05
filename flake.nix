{
  description = "Optimus";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flakeUtils.url = "github:numtide/flake-utils";
    nix2container.url = "github:nlewo/nix2container";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flakeUtils";
      };
    };
  };

  outputs =
    { self, nixpkgs, flakeUtils, nix2container, rust-overlay, ... }@inputs:
    flakeUtils.lib.eachSystem [ "x86_64-linux" ] (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
          config.allowUnfree = true;
        };

        languagePackages = with pkgs; [
          rust-bin.beta.latest.default
          rust-analyzer
          clippy
          cargo-watch
          cargo-nextest
          nushell
        ];
      in {
        devShells.default = pkgs.mkShell {
          packages = languagePackages;
          # RUST_SRC_PATH =
          #   "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
      });
}


{
  description = "Static site generator with JSX-like syntax, Rhai scripting, and more.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, fenix, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ fenix.overlays.default ];
      };

      toolchain = fenix.packages.${system}.minimal.toolchain;
    in {
      packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
        pname = "poet";
        version = "0.3.0";

        src = ./.;

        cargoLock = {
          lockFile = ./Cargo.lock;
        };

        nativeBuildInputs = [
          toolchain
          pkgs.pkg-config
          pkgs.rustPlatform.bindgenHook
        ];
      };

      devShells.${system}.default = pkgs.mkShell {
        buildInputs = [
          toolchain
          pkgs.pkg-config
        ];
      };
    };
}

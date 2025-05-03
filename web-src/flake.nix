{
  description = "chip-8-emulator web-src flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      rec {
        flakedPkgs = pkgs;
        formatter = pkgs.nixfmt-rfc-style;

        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            nodejs_22
            corepack
          ];
        };
      }
    );
}

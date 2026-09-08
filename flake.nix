{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    ...
  }: let
    pkgs = import nixpkgs {
      system = "x86_64-linux";
      overlays = [(import rust-overlay)];
    };
  in {
    packages.x86_64-linux.hello = pkgs.hello;
    devShells.x86_64-linux.default = pkgs.mkShell {
      name = ":=)";
      packages = with pkgs; [
        rust-bin.stable.latest.default
        rust-analyzer
        nix
        rustfmt
        file
      ];
    };
  };
}

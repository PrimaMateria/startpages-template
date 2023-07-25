{
  description = "GitHub Simple Startpage Generator";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = inputs@{ self, ... }:
    inputs.flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import inputs.nixpkgs {
          inherit system;
        };
      in
      {
        devShells = rec {
          rust = pkgs.mkShell {
            buildInputs = [
              pkgs.rustc
              pkgs.cargo
            ];
          };
          default = rust;
        };
      });
}

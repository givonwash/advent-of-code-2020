{
  description = "Advent of Code 2020 Solutions in Rust!";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-22.05";
    flake-utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, flake-utils, naersk, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit overlays system; };
        naersk-lib = naersk.lib.${system};
      in
      rec {
        packages =
          let
            inherit (builtins) mapAttrs match readDir;
            inherit (pkgs.lib) attrsets;
          in
          (mapAttrs
            (name: _: naersk-lib.buildPackage { pname = name; root = ./${name}; })
            (attrsets.filterAttrs
              (entry: type: (match "^day[012][0-9]$" entry) == [ ] && type == "directory")
              (readDir ./.)));
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rust-bin.stable.latest.default
          ];
        };
      });
}

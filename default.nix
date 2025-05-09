{pkgs ? import <nixpkgs> {}}: let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
  pkgs.rustPlatform.buildRustPackage {
    pname = manifest.name;
    version = manifest.version;
    cargoLock.lockFile = ./Cargo.lock;

    cargoLock.outputHashes = {
      "crokey-1.1.2" = "sha256-BqvG05nmSUp20/xLZjBjs9kjUI4sBDNdqCCnTp06+SY=";
    };

    src = pkgs.lib.cleanSource ./.;
  }

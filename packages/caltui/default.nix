{ pkgs
, lib
}:
let
  manifest = (lib.importTOML ./Cargo.toml).package;
  workspaceManifest = (lib.importTOML ../../Cargo.toml).workspace.package;
in
pkgs.rustPlatform.buildRustPackage rec {
  pname = manifest.name;
  version = workspaceManifest.version;

  nativeBuildInputs = with pkgs; [
    pkg-config
  ];

  buildInputs = with pkgs; [
    alsa-lib
    dbus
  ];

  cargoBuildFlags = [ "--package" pname ];

  src = lib.sourceByRegex ../.. [
    "^Cargo.toml$"
    "^Cargo.lock$"
    "^packages.*$"
  ];

  cargoLock.lockFile = ../../Cargo.lock;
}

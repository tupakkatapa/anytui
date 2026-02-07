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
    makeWrapper
  ];

  buildInputs = with pkgs; [
    alsa-lib
    dbus
  ];

  postInstall = ''
    wrapProgram $out/bin/${pname} \
      --prefix PATH : ${lib.makeBinPath [ pkgs.systemd pkgs.iwd pkgs.wpa_supplicant pkgs.networkmanager ]}
  '';

  cargoBuildFlags = [ "--package" pname ];

  src = lib.sourceByRegex ../.. [
    "^Cargo.toml$"
    "^Cargo.lock$"
    "^packages.*$"
  ];

  cargoLock.lockFile = ../../Cargo.lock;
}

{
  nixConfig = {
    extra-substituters = [
      "https://cache.nixos.org"
      "https://nix-community.cachix.org"
    ];
    extra-trusted-public-keys = [
      "cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY="
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    ];
  };

  inputs = {
    devenv.url = "github:cachix/devenv";
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
  };

  outputs = { self, ... }@inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = inputs.nixpkgs.lib.systems.flakeExposed;
      imports = [
        inputs.devenv.flakeModule
        inputs.flake-parts.flakeModules.easyOverlay
      ];

      perSystem =
        { pkgs
        , system
        , ...
        }:
        let
          packages = {
            mustui = pkgs.callPackage ./packages/mustui/default.nix { };
            blutui = pkgs.callPackage ./packages/blutui/default.nix { };
            nettui = pkgs.callPackage ./packages/nettui/default.nix { };
            voltui = pkgs.callPackage ./packages/voltui/default.nix { };
            kaltui = pkgs.callPackage ./packages/kaltui/default.nix { };
            caltui = pkgs.callPackage ./packages/caltui/default.nix { };
            tuigreat = pkgs.callPackage ./packages/tuigreat/default.nix { };
          };
        in
        {
          # Overlays
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
              self.overlays.default
            ];
            config = { };
          };
          overlayAttrs = packages;

          # Development shell -> 'nix develop' or 'direnv allow'
          devenv.shells = {
            default = {
              packages = with pkgs; [
                cargo-tarpaulin
                pkg-config
                alsa-lib
                dbus
              ];
              languages.rust = {
                enable = true;
                components = [ "cargo" "clippy" "rustfmt" ];
              };
              git-hooks.hooks = {
                nixpkgs-fmt.enable = true;
                rustfmt.enable = true;
                pedantic-clippy = {
                  enable = true;
                  entry = "cargo clippy -- -D clippy::pedantic";
                  files = "\\.rs$";
                  pass_filenames = false;
                };
                cargo-test = {
                  enable = true;
                  entry = "cargo test --all-features";
                  files = "\\.rs$";
                  pass_filenames = false;
                };
              };
              # Workaround for https://github.com/cachix/devenv/issues/760
              containers = pkgs.lib.mkForce { };
            };
          };

          # Custom packages and entrypoint aliases -> 'nix run' or 'nix build'
          inherit packages;
        };
    };
}

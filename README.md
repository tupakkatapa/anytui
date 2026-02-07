# anytui

Sick and tired of visually non-coherent GUI tools and managers which pull god-knows-what packages as their deps, not a fan of complicated CLI tools either. Luckily **anything can be TUI.**

So here is some minimal TUI managers and tools for graphical desktop environments. Built with [Ratatui](https://ratatui.rs/) and a shared library for consistent look and feel.

## Packages

- **voltui** - Audio volume control via `pactl` (PipeWire/PulseAudio) or `amixer` (ALSA)
- **nettui** - Network manager via `iwctl` (iwd), `wpa_cli`, or `nmcli` (NetworkManager)
- **blutui** - Bluetooth manager (requires `bluetooth` service)
- **mustui** - Music player with MPRIS support (requires D-Bus)
- **caltui** - Calendar viewer
- **kaltui** - Calculator

Some packages require system services. Both `nettui` and `voltui` detect available backends at runtime -- no compile-time configuration needed. All tools share common vim-style keybindings.

## Getting Started

Add this repository as a Nix flake input and apply the overlay to make packages available:

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    anytui.url = "github:tupakkatapa/anytui";
    anytui.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, anytui, ... }: {
    nixosConfigurations.yourhostname = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        ./configuration.nix
        {
          nixpkgs.overlays = [ anytui.overlays.default ];
          environment.systemPackages = with pkgs; [
            voltui
            nettui
            blutui
            mustui
            caltui
            kaltui
          ];
        }
      ];
    };
  };
}
```

Alternatively, reference packages directly without an overlay:

```nix
environment.systemPackages = with anytui.packages.x86_64-linux; [
  voltui
  nettui
];
```

## Usage

```bash
# Try without installing
nix run github:tupakkatapa/anytui#voltui
nix run github:tupakkatapa/anytui#nettui

# Common keybindings across all tools
# j/k     - Navigate up/down
# g/G     - Jump to top/bottom
# /       - Search
# q/Esc   - Quit
# ?       - Help
```

## Contributing

```bash
nix develop           # Enter dev shell (or use direnv)
cargo build           # Build all packages
pre-commit run -a     # Run lints (nixpkgs-fmt, rustfmt, clippy)
```

Pre-commit hooks are automatically installed via devenv.

# anytui

Sick and tired of visually non-coherent GUI tools and managers which pull god-knows-what packages as their deps, not a fan of complicated CLI tools either.

Luckily **Anything can be TUI.**

Minimal terminal user interface managers and tools for graphical desktop environments. Built with [Ratatui](https://ratatui.rs/) and a shared library for consistent look and feel.

## Packages

- **voltui** - Audio volume control (PipeWire/PulseAudio/ALSA)
- **nettui** - Network manager (iwd/wpa_supplicant/NetworkManager)
- **blutui** - Bluetooth manager
- **mustui** - Music player with MPRIS support
- **caltui** - Calendar viewer
- **kaltui** - Calculator

All tools share common vim-style keybindings through the `tuigreat` shared library.

## Runtime Requirements

Some packages require system services. Both `nettui` and `voltui` detect available backends at runtime -- no compile-time configuration needed.

- **voltui** - One of: `PipeWire`/`PulseAudio` (pactl) or `ALSA` (amixer)
- **nettui** - One of: `iwd` (iwctl), `wpa_supplicant` (wpa_cli), or `NetworkManager` (nmcli)
- **blutui** - `bluetooth` service enabled
- **mustui** - Audio output (ALSA), D-Bus for MPRIS

## Getting Started

Add this repository as a Nix flake input and apply the overlay to make packages available:

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
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
# Run any tool directly
voltui
nettui
mustui ~/Music

# Common keybindings across all tools
# j/k     - Navigate up/down
# g/G     - Jump to top/bottom
# /       - Search
# q/Esc   - Quit
# ?       - Help
```

## Building from Source

```bash
nix build .#voltui    # Build a single package
nix develop           # Enter development shell
cargo build           # Build all packages
```

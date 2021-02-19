# Nixos-update-checker
## Usage
```bash
$ nixos-update-checker [-f flake.lock] [ <channel-name> ... ]
```
## Example
```bash
$ nixos-update-checker nixos-unstable nixos-20.09-small
```

## Building
### Via Cargo
```bash
$ cargo build
```

### As a nix flake
```bash
$ nix build
```

## Install
### Via Cargo
```bash
$ cargo install -f --path .
```
### As a nix flake
Add this repository as a flake to your flake.nix, e.g.
```nix
{
  description = ...;
  ...
  inputs.nixos-update-checker.url = "github:BurNiinTRee/nixos-update-checker";
  ...
}
```
and add it to your system packages.


## License
This program is available under the GNU GPLv3 or later.

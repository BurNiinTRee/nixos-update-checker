{
  description = "My very special Rust project";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    import-cargo.url = "github:edolstra/import-cargo";
    nixpkgs.url = "nixpkgs/nixos-unstable";
    mozilla = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
  };

  outputs = { self, flake-utils, import-cargo, nixpkgs, mozilla }: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
      mozpkgs = pkgs.callPackage (mozilla + "/package-set.nix") { };
      rust = (mozpkgs.rustChannelOf {
        date = "2021-02-19";
        channel = "nightly";
        sha256 = "sha256-pHxt+P6JSPKkPXxWz9wA91lMhdRZyfhlAA9VZWyirDw=";
      }).rust;
      cargo-vendor = (import-cargo.builders.importCargo {
        lockFile = ./Cargo.lock;
        inherit pkgs;
      });
      buildRelease = isRelease: pkgs.stdenv.mkDerivation {
        pname = "nixos-update-checker";
        version = "0.1.0";
        src = self;
        nativeBuildInputs = [
          cargo-vendor.cargoHome
          rust
        ] ++ (with pkgs; [ openssl pkgconfig ]);
        buildPhase = ''
          cargo build --offline ${if isRelease then "--release" else ""}
        '';
        installPhase = ''
          install -Dt $out/bin ./target/${if isRelease then "release" else "debug"}/hello
        '';
        shellHook = ''
          unset CARGO_HOME
        '';
      };
    in
    rec {
      packages = {
        debug = buildRelease false;
        release = buildRelease true;
      };
      defaultPackage = packages.release;

    });
}

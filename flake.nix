{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};

    nativeBuildInputs = with pkgs; [
      rustc
      clippy
      cargo
      meson
      ninja
      pkg-config
      wrapGAppsHook4
      blueprint-compiler
      rustPlatform.cargoSetupHook
    ];

    buildInputs = with pkgs; [
      gtk4
      libadwaita
      gtk4-layer-shell
    ];

    pname = "shell";
    version = "0.1.0";
    src = ./.;
  in {
    devShells.${system}.default = pkgs.mkShell {
      inherit nativeBuildInputs buildInputs;
      RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      
      packages = with pkgs; [
        flatpak-builder
        rust-analyzer
        rustfmt
        d-spy
        ghex
      ];
    };

    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = pname;
      inherit version src;

      cargoDeps = pkgs.rustPlatform.fetchCargoVendor {
        inherit pname version src;
        hash = "sha256-s8cEWMsSGtSxgYNzx02eG52k36v1wbSKSf4omNEdqFc=";
      };

      inherit nativeBuildInputs buildInputs;
    };
  };
}

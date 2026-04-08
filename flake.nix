{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    astal = {
      url = "github:aylur/astal";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, astal }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};

    astalpkgs = with astal.packages.${system}; [
      apps
    ];

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
    ] ++ astalpkgs;

    pname = "rshell";
    version = "0.1.0";
    src = ./.;
  in {
    devShells.${system}.default = pkgs.mkShell {
      inherit nativeBuildInputs buildInputs;
      RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      
      packages = with pkgs; [
        rust-analyzer
        rustfmt
      ];
    };

    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = pname;
      inherit version src;

      cargoDeps = pkgs.rustPlatform.fetchCargoVendor {
        inherit pname version src;
        hash = "sha256-pqA1NJzpKr4Sjonclt5xDu/6QhX59KgCmQw6oijYApQ=";
      };

      inherit nativeBuildInputs buildInputs;
    };
  };
}

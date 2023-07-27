{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay/master";
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; config.allowUnfree = true; };
      in
      {
        devShell = pkgs.mkShell
          rec {
            buildInputs =
              [
                (pkgs.rust-bin.nightly."2023-03-06".default.override {
                  extensions = [ "rust-src" ];
                })
                #pkgs.protobuf
                pkgs.clang
                pkgs.libclang
                pkgs.openssl
                pkgs.pkg-config
                pkgs.linuxPackages.nvidia_x11
                pkgs.cudatoolkit
              ];
            #PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
            LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
            CUDA_LIBRARY_PATH = pkgs.cudatoolkit;
            #LD_LIBRARY_PATH="${pkgs.python3Packages.lib}/lib";

          };
      });
}


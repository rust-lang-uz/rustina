# For more, refer to:
# https://github.com/NixOS/nixpkgs/blob/master/doc/languages-frameworks/rust.section.md
{pkgs ? import <nixpkgs> {}}: let
  lib = pkgs.lib;
  getLibFolder = pkg: "${pkg}/lib";
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
  pkgs.rustPlatform.buildRustPackage {
    pname = "bot";
    version = manifest.version;
    cargoLock.lockFile = ./Cargo.lock;
    src = pkgs.lib.cleanSource ./.;

    nativeBuildInputs = with pkgs; [
      # Rust
      rustc
      cargo

      # LLVM & GCC
      gcc
      cmake
      gnumake
      pkg-config
      llvmPackages.llvm
      llvmPackages.clang
    ];

    buildInputs = with pkgs; [
      openssl
      libressl
    ];

    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
      pkgs.gcc
      pkgs.libiconv
      pkgs.llvmPackages.llvm
    ];

    # If you wanna get thorny
    # RUST_BACKTRACE = 1;
    NIX_LDFLAGS = "-L${(getLibFolder pkgs.libiconv)}";
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

    meta = with lib; {
      homepage = manifest.homepage;
      description = "Telegram bot manager for Uzbek Rust community";
      license = with lib.licenses; [asl20];

      platforms = with platforms; linux ++ darwin;

      maintainers = [
        {
          name = "Sokhibjon Orzikulov";
          email = "sakhib@orzklv.uz";
          handle = "orzklv";
          github = "orzklv";
          githubId = 54666588;
          keys = [
            {
              fingerprint = "00D2 7BC6 8707 0683 FBB9  137C 3C35 D3AF 0DA1 D6A8";
            }
          ];
        }
      ];
    };
  }

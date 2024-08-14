{ pkgs ? import <nixpkgs> {} }:
let
  getLibFolder = pkg: "${pkg}/lib";
  getFramwork = pkg: "${pkg}/Library/Frameworks";
  darwinOptions = if pkgs.stdenv.isDarwin then ''
    -F${(getFramwork pkgs.darwin.apple_sdk.frameworks.Security)}
    -F${(getFramwork pkgs.darwin.apple_sdk.frameworks.CoreFoundation)}
    -F${(getFramwork pkgs.darwin.apple_sdk.frameworks.CoreServices)}
    -F${(getFramwork pkgs.darwin.apple_sdk.frameworks.SystemConfiguration)}
  '' else "";
in
pkgs.stdenv.mkDerivation {
  name = "telegram";

  nativeBuildInputs = [
    pkgs.gcc
    pkgs.rustc
    pkgs.cargo
    pkgs.cargo-watch
    pkgs.pkg-config
    pkgs.rust-analyzer
    pkgs.llvmPackages.llvm
    pkgs.llvmPackages.clang
  ];

  buildInputs = [
    pkgs.openssl
    pkgs.darwin.apple_sdk.frameworks.Security
    pkgs.darwin.apple_sdk.frameworks.CoreServices
    pkgs.darwin.apple_sdk.frameworks.CoreFoundation
    pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;
  NIX_LDFLAGS = "-L${(getLibFolder pkgs.libiconv)} ${darwinOptions}";
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
    (getLibFolder pkgs.gcc)
    (getLibFolder pkgs.libiconv)
    (getLibFolder pkgs.llvmPackages.llvm)
  ];

  shellHook = ''
    # Load the environment variables from the .env file
    if [ ! -f .env ]; then
      echo "Please enter your telegram bot token: ";
      read -r TELOXIDE_TOKEN;
      echo "TELOXIDE_TOKEN=$TELOXIDE_TOKEN" > .env;
    else
      source .env;
    fi

    # Set the environment variable
    export TELOXIDE_TOKEN=$TELOXIDE_TOKEN;

    # Start watching for changes
    # Start watching for changes in the background
    # cargo watch -x "run" &

    # Store the PID of the background process
    # CARGO_WATCH_PID=$!

    # Function to clean up the background process on exit
    # cleanup() {
    #   kill $CARGO_WATCH_PID
    # }

    # Trap EXIT signal to run cleanup function
    # trap cleanup EXIT
  '';
}

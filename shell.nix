{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustup
    pkgs.cargo-udeps
    pkgs.cargo-machete
    pkgs.openssl
    pkgs.pkg-config
    pkgs.clang
    pkgs.gnumake           
    pkgs.protobuf          
    pkgs.gcc
  ];

  LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
  # LD_LIBRARY_PATH="${stdenv.cc.cc.lib}/lib64:$LD_LIBRARY_PATH";
  # LD_LIBRARY_PATH="${pkgs.gcc.cc.libc}/lib64:$LD_LIBRARY_PATH";
  LD_LIBRARY_PATH="${pkgs.gcc.cc.lib}/lib64:$LD_LIBRARY_PATH";

  shellHook = ''
    # Set the Rust toolchain to stable and install necessary components
    rustup default stable --profile minimal
    rustup component add rustfmt clippy rust-src
    rustup target add wasm32-unknown-unknown

    # Install zepter
    # cargo install zepter -f --locked
    # cargo install cargo-machete

    # rustup component add rustc-dev llvm-tools-preview
  '';
}
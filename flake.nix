{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { nixpkgs, flake-utils, ... }:
let
  supported-systems = with flake-utils.lib.system; [
    x86_64-linux
  ];
in
flake-utils.lib.eachSystem supported-systems (system:
let
  pkgs = import nixpkgs { inherit system; };
in
{
  devShells = {
    default = pkgs.mkShell {
      nativeBuildInputs = with pkgs; [
        aravis
	glib
        cmake
        cmake-format
        clang
        clang-tools
        nixpkgs-fmt
        openssl
        pkg-config
        rustc
        cargo 
        rustfmt
	gir-rs
	fontconfig
      ];
      LIBCLANG_PATH = pkgs.lib.optionalString pkgs.stdenv.isLinux "${pkgs.libclang.lib}/lib/";
    };
  };
});
}

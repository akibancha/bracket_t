{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  name = "bracket_t rust shell";
  buildInputs = with pkgs; [
        cargo
        gcc
        rust-analyzer
        pkgconfig
        fontconfig
        cmake
    ];
}

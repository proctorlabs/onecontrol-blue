{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell
{
  buildInputs = [
    pkgs.stdenv
    pkgs.rustup
    pkgs.pkgconfig
    pkgs.dbus
  ];

  shellHook = ''
    rustup toolchain install stable
  '';
}

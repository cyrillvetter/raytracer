{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell rec {
  nativeBuildInputs = [
    pkg-config
  ];
  buildInputs = [
    xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr
  ];
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}

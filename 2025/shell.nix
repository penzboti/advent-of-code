{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    # nativeBuildInputs is usually what you want -- tools you need to run
    nativeBuildInputs = with pkgs.buildPackages; 
    [
      clang-tools
      libgcc

      # not enough fiddling will fix iostream not found by clangd
      # many options tried using these combinations:
      # well at least not for a VERY minimalistic workflow
      # pkgs.gcc
      # bear
      # gnumake
      # cmake
    ];

    shellHook = ''
      alias build="g++ main.cpp -o main && echo 'built'";
      alias run="build && ./main";
    '';
}

{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
    buildInputs = with pkgs; [
        cargo rustfmt rust-analyzer
    ];
}

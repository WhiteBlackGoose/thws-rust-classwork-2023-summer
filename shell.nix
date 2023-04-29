let
  pkgs = import (fetchTarball("https://github.com/NixOS/nixpkgs/archive/acc86a93168e272538f4ce459eaef3f58848ebd0.tar.gz")) {};

in pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    rustc
    rust-analyzer
    vscode-extensions.vadimcn.vscode-lldb
    openssl cmake zlib
  ];
  OPENSSL_DIR="${pkgs.openssl.dev}";
  OPENSSL_LIB_DIR="${pkgs.openssl.out}/lib";
  VSCODE_CODELLDB = "${pkgs.vscode-extensions.vadimcn.vscode-lldb}";
}

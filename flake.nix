# Borrowed some stuff from https://ayats.org/blog/nix-rustup/
{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay"; 

  outputs = { nixpkgs, rust-overlay, ... }:
      let 
        systems = [ "aarch64-darwin" "x86_64-darwin" "aarch64-linux" "x86_64-linux" ]; 
      in {
        devShells = nixpkgs.lib.genAttrs systems (system: 
        let 
          pkgs = import nixpkgs {
            inherit system;
            overlays = [rust-overlay.overlays.default];
          };
          toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;
          in
        {
          default =
          pkgs.mkShell {
            buildInputs = with pkgs; [
              toolchain
              rust-analyzer-unwrapped
              wasm-pack
              vscode-extensions.vadimcn.vscode-lldb
              openssl cmake zlib
              (
                let 
                  py = python3.withPackages(p: [ p.httpserver ]);
                  in 
                (writeScriptBin "server" "${py}/bin/python -m http.server")
              )
            ];
            VSCODE_CODELLDB = "${pkgs.vscode-extensions.vadimcn.vscode-lldb}";
            OPENSSL_DIR="${pkgs.openssl.dev}";
            OPENSSL_LIB_DIR="${pkgs.openssl.out}/lib";
            RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
          };
        });
    };
}

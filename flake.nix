{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

  outputs = { nixpkgs, ... }:
      let 
        systems = [ "aarch64-darwin" "x86_64-darwin" "aarch64-linux" "x86_64-linux" ]; 
      in {
        devShells = nixpkgs.lib.genAttrs systems (system: 
        let 
          pkgs = nixpkgs.legacyPackages.${system}; in
        {
          default =
          pkgs.mkShell {
            buildInputs = with pkgs; [
              cargo
              rustc
              rust-analyzer
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
          };
        });
    };
}

{
  description = "Koudaisai portal development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            # Rust
            rustc
            cargo
            rustfmt
            clippy
            cargo-watch
            sqlx-cli
            pkg-config
            openssl

            # Node.js
            nodejs_24

            # Useful tools
            caddy
            git
            openapi-generator-cli
          ];

          shellHook = ''
                      corepack enable pnpm >/dev/null 2>&1 || true
                      export NX_SOCKET_DIR="/tmp/koudaisai-portal-nx"

                      echo "dev shell"
                      echo "node: $(node --version)"
                      echo "npm: $(npm --version)"
                      echo "cargo: $(cargo --version)"
                      echo "NX_SOCKET_DIR: $NX_SOCKET_DIR"

                      if ! docker info >/dev/null 2>&1; then
                        echo ""
                        echo "警告: Docker daemon が起動していません。"
                        echo "Docker Desktop を起動してください。"
                        echo ""
                      fi
                    '';
        };
      });
}

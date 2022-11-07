{
  description = "hcc-server build flake";

  inputs = {
    rust-overlay = { url = "github:oxalica/rust-overlay"; };
  };

  outputs = { nixpkgs, rust-overlay, ... }:
    let system = "x86_64-linux";
    in {
      devShell.${system} = let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlay ];
        };
      in (({ pkgs, ... }:
        pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            cargo-watch
            pkg-config
            glibc
            openssl
            openssl.bin
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" ];
            })
          ];
              
           shellHook = "
                mkdir -p .secrets && touch .secrets/.env
          ";   

        }) { pkgs = pkgs; });
    };
}
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
            openssl
            openssl.bin
          ];
              
           shellHook = "
                mkdir -p .secrets && touch .secrets/.env
          ";   

        }) { pkgs = pkgs; });
    };
}
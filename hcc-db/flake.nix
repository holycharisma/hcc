{
  description = "hcc build flake";

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
            pkg-config
            openssl
          ];

          shellHook = "
                stat .cargo-nix-local/bin/sea > /dev/null && echo 'sea-orm-cli installed' || \ 
                cargo install sea-orm-cli --bin sea --root .cargo-nix-local/
          ";
        }) { pkgs = pkgs; });
    };
}
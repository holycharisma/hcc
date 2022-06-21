{
  description = "hcc-server build flake";

  inputs = {  };

  outputs = { nixpkgs, ... }:
    let system = "x86_64-linux";
    in {
      devShell.${system} = let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ ];
        };
      in (({ pkgs, ... }:
        pkgs.mkShell {
          buildInputs = with pkgs; [
            pkg-config
            openssl
            openssl.bin
          ];
              
           shellHook = "
                mkdir -p .secrets && touch .secrets/.env
                stat .secrets/jwtRS256.key > /dev/null || ssh-keygen -t rsa -b 4096 -m PEM -f ./.secrets/jwtRS256.key
                stat .secrets/jwtRS256.key.pub > /dev/null || openssl rsa -in jwtRS256.key -pubout -outform PEM -out ./.secrets/jwtRS256.key.pub
          ";   

        }) { pkgs = pkgs; });
    };
}
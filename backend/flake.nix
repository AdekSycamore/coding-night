{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { 
    self, 
    nixpkgs, 
    utils, 
    rust-overlay 
    } @ inputs: utils.lib.eachDefaultSystem (system:
      let
        overlays = [
          rust-overlay.overlays.default
          (final: prev: {
            rustToolchain = final.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          })
        ];

        pkgs = import nixpkgs { inherit system overlays; };
      in
      {
        devShell = with pkgs; mkShell {
          buildInputs = [ 
            cowsay 
            rustup  
            diesel-cli 
            postgresql
            rustToolchain
          ];

          shellHook = ''
            cowsay "Paweł to gej"
          '';
        };
        
      });
}
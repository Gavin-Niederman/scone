{
  description = "A very basic flake";

  outputs = { self, nixpkgs }: 
  with nixpkgs; 
  {

    devShells.x86_64-linux.default = 
      let
        pkgs = nixpkgs.legacyPackages.x86_64-linux; 
      in pkgs.mkShell {
        buildInputs = [
          pkgs.rustup
          pkgs.cmake
          pkgs.pkg-config
          pkgs.fontconfig
        ];
      };
  };
}

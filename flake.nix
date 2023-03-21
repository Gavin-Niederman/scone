{
  description = "A very basic flake";

  outputs = { self, nixpkgs }: 
  with nixpkgs; 
  {

    devShells.x86_64-linux.default = 
      let
        pkgs = nixpkgs.legacyPackages.x86_64-linux; 
      in pkgs.mkShell {
        buildInputs = with pkgs; [
          pkgconfig
          xorg.libX11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          wayland
          cargo-flamegraph
        ];
      };
  };
}
